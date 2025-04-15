# Crate Documentation

**Version:** 1.10.0

**Format Version:** 43

# Module `rayon`

Rayon is a data-parallelism library that makes it easy to convert sequential
computations into parallel.

It is lightweight and convenient for introducing parallelism into existing
code. It guarantees data-race free executions and takes advantage of
parallelism when sensible, based on work-load at runtime.

# How to use Rayon

There are two ways to use Rayon:

- **High-level parallel constructs** are the simplest way to use Rayon and also
  typically the most efficient.
  - [Parallel iterators][iter module] make it easy to convert a sequential iterator to
    execute in parallel.
    - The [`ParallelIterator`] trait defines general methods for all parallel iterators.
    - The [`IndexedParallelIterator`] trait adds methods for iterators that support random
      access.
  - The [`par_sort`] method sorts `&mut [T]` slices (or vectors) in parallel.
  - [`par_extend`] can be used to efficiently grow collections with items produced
    by a parallel iterator.
- **Custom tasks** let you divide your work into parallel tasks yourself.
  - [`join`] is used to subdivide a task into two pieces.
  - [`scope`] creates a scope within which you can create any number of parallel tasks.
  - [`ThreadPoolBuilder`] can be used to create your own thread pools or customize
    the global one.

[iter module]: iter/index.html
[`join`]: fn.join.html
[`scope`]: fn.scope.html
[`par_sort`]: slice/trait.ParallelSliceMut.html#method.par_sort
[`par_extend`]: iter/trait.ParallelExtend.html#tymethod.par_extend
[`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html

# Basic usage and the Rayon prelude

First, you will need to add `rayon` to your `Cargo.toml`.

Next, to use parallel iterators or the other high-level methods,
you need to import several traits. Those traits are bundled into
the module [`rayon::prelude`]. It is recommended that you import
all of these traits at once by adding `use rayon::prelude::*` at
the top of each module that uses Rayon methods.

These traits give you access to the `par_iter` method which provides
parallel implementations of many iterative functions such as [`map`],
[`for_each`], [`filter`], [`fold`], and [more].

[`rayon::prelude`]: prelude/index.html
[`map`]: iter/trait.ParallelIterator.html#method.map
[`for_each`]: iter/trait.ParallelIterator.html#method.for_each
[`filter`]: iter/trait.ParallelIterator.html#method.filter
[`fold`]: iter/trait.ParallelIterator.html#method.fold
[more]: iter/trait.ParallelIterator.html#provided-methods
[`ParallelIterator`]: iter/trait.ParallelIterator.html
[`IndexedParallelIterator`]: iter/trait.IndexedParallelIterator.html

# Crate Layout

Rayon extends many of the types found in the standard library with
parallel iterator implementations. The modules in the `rayon`
crate mirror [`std`] itself: so, e.g., the `option` module in
Rayon contains parallel iterators for the `Option` type, which is
found in [the `option` module of `std`]. Similarly, the
`collections` module in Rayon offers parallel iterator types for
[the `collections` from `std`]. You will rarely need to access
these submodules unless you need to name iterator types
explicitly.

[the `option` module of `std`]: https://doc.rust-lang.org/std/option/index.html
[the `collections` from `std`]: https://doc.rust-lang.org/std/collections/index.html
[`std`]: https://doc.rust-lang.org/std/

# Targets without threading

Rayon has limited support for targets without `std` threading implementations.
See the [`rayon_core`] documentation for more information about its global fallback.

# Other questions?

See [the Rayon FAQ][faq].

[faq]: https://github.com/rayon-rs/rayon/blob/main/FAQ.md

## Modules

## Module `array`

Parallel iterator types for [arrays] (`[T; N]`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[arrays]: https://doc.rust-lang.org/std/primitive.array.html

```rust
pub mod array { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator that moves out of an array.

```rust
pub struct IntoIter<T: Send, const N: usize> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T, N> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
## Module `collections`

Parallel iterator types for [standard collections][std::collections]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[std::collections]: https://doc.rust-lang.org/stable/std/collections/

```rust
pub mod collections { /* ... */ }
```

### Modules

## Module `binary_heap`

This module contains the parallel iterator types for heaps
(`BinaryHeap<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod binary_heap { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a binary heap

```rust
pub struct IntoIter<T: Ord + Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Iter`

Parallel iterator over an immutable reference to a binary heap

```rust
pub struct Iter<''a, T: Ord + Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Drain`

Draining parallel iterator that moves out of a binary heap,
but keeps the total capacity.

```rust
pub struct Drain<''a, T: Ord + Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

## Module `btree_map`

This module contains the parallel iterator types for B-Tree maps
(`BTreeMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod btree_map { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a B-Tree map

```rust
pub struct IntoIter<K: Ord + Send, V: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Sync**
- **Unpin**
- **IntoEither**
- **RefUnwindSafe**
- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Iter`

Parallel iterator over an immutable reference to a B-Tree map

```rust
pub struct Iter<''a, K: Ord + Sync, V: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
#### Struct `IterMut`

Parallel iterator over a mutable reference to a B-Tree map

```rust
pub struct IterMut<''a, K: Ord + Sync, V: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
## Module `btree_set`

This module contains the parallel iterator types for B-Tree sets
(`BTreeSet<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod btree_set { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a B-Tree set

```rust
pub struct IntoIter<T: Ord + Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Send**
- **Sync**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Iter`

Parallel iterator over an immutable reference to a B-Tree set

```rust
pub struct Iter<''a, T: Ord + Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `hash_map`

This module contains the parallel iterator types for hash maps
(`HashMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod hash_map { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a hash map

```rust
pub struct IntoIter<K: Hash + Eq + Send, V: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

#### Struct `Iter`

Parallel iterator over an immutable reference to a hash map

```rust
pub struct Iter<''a, K: Hash + Eq + Sync, V: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

#### Struct `IterMut`

Parallel iterator over a mutable reference to a hash map

```rust
pub struct IterMut<''a, K: Hash + Eq + Sync, V: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Unpin**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Drain`

Draining parallel iterator that moves out of a hash map,
but keeps the total capacity.

```rust
pub struct Drain<''a, K: Hash + Eq + Send, V: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `hash_set`

This module contains the parallel iterator types for hash sets
(`HashSet<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod hash_set { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a hash set

```rust
pub struct IntoIter<T: Hash + Eq + Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **IntoEither**
#### Struct `Iter`

Parallel iterator over an immutable reference to a hash set

```rust
pub struct Iter<''a, T: Hash + Eq + Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Drain`

Draining parallel iterator that moves out of a hash set,
but keeps the total capacity.

```rust
pub struct Drain<''a, T: Hash + Eq + Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `linked_list`

This module contains the parallel iterator types for linked lists
(`LinkedList<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod linked_list { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a linked list

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Iter`

Parallel iterator over an immutable reference to a linked list

```rust
pub struct Iter<''a, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Sync**
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

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `IterMut`

Parallel iterator over a mutable reference to a linked list

```rust
pub struct IterMut<''a, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `vec_deque`

This module contains the parallel iterator types for double-ended queues
(`VecDeque<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod vec_deque { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a double-ended queue

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

#### Struct `Iter`

Parallel iterator over an immutable reference to a double-ended queue

```rust
pub struct Iter<''a, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **Sync**
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

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Freeze**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `IterMut`

Parallel iterator over a mutable reference to a double-ended queue

```rust
pub struct IterMut<''a, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Sync**
- **Send**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Drain`

Draining parallel iterator that moves a range out of a double-ended queue,
but keeps the total capacity.

```rust
pub struct Drain<''a, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Sync**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `iter`

Traits for writing parallel programs using an iterator-style interface

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

Parallel iterators make it easy to write iterator-like chains that
execute in parallel: typically all you have to do is convert the
first `.iter()` (or `iter_mut()`, `into_iter()`, etc) method into
`par_iter()` (or `par_iter_mut()`, `into_par_iter()`, etc). For
example, to compute the sum of the squares of a sequence of
integers, one might write:

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|i| i * i)
         .sum()
}
```

Or, to increment all the integers in a slice, you could write:

```rust
use rayon::prelude::*;
fn increment_all(input: &mut [i32]) {
    input.par_iter_mut()
         .for_each(|p| *p += 1);
}
```

To use parallel iterators, first import the traits by adding
something like `use rayon::prelude::*` to your module. You can
then call `par_iter`, `par_iter_mut`, or `into_par_iter` to get a
parallel iterator. Like a [regular iterator][], parallel
iterators work by first constructing a computation and then
executing it.

In addition to `par_iter()` and friends, some types offer other
ways to create (or consume) parallel iterators:

- Slices (`&[T]`, `&mut [T]`) offer methods like `par_split` and
  `par_windows`, as well as various parallel sorting
  operations. See [the `ParallelSlice` trait] for the full list.
- Strings (`&str`) offer methods like `par_split` and `par_lines`.
  See [the `ParallelString` trait] for the full list.
- Various collections offer [`par_extend`], which grows a
  collection given a parallel iterator. (If you don't have a
  collection to extend, you can use [`collect()`] to create a new
  one from scratch.)

[the `ParallelSlice` trait]: ../slice/trait.ParallelSlice.html
[the `ParallelString` trait]: ../str/trait.ParallelString.html
[`par_extend`]: trait.ParallelExtend.html
[`collect()`]: trait.ParallelIterator.html#method.collect

To see the full range of methods available on parallel iterators,
check out the [`ParallelIterator`] and [`IndexedParallelIterator`]
traits.

If you'd like to build a custom parallel iterator, or to write your own
combinator, then check out the [split] function and the [plumbing] module.

[regular iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`ParallelIterator`]: trait.ParallelIterator.html
[`IndexedParallelIterator`]: trait.IndexedParallelIterator.html
[split]: fn.split.html
[plumbing]: plumbing/index.html

Note: Several of the `ParallelIterator` methods rely on a `Try` trait which
has been deliberately obscured from the public API.  This trait is intended
to mirror the unstable `std::ops::Try` with implementations for `Option` and
`Result`, where `Some`/`Ok` values will let those iterators continue, but
`None`/`Err` values will exit early.

A note about object safety: It is currently _not_ possible to wrap
a `ParallelIterator` (or any trait that depends on it) using a
`Box<dyn ParallelIterator>` or other kind of dynamic allocation,
because `ParallelIterator` is **not object-safe**.
(This keeps the implementation simpler and allows extra optimizations.)

```rust
pub mod iter { /* ... */ }
```

### Modules

## Module `plumbing`

Traits and functions used to implement parallel iteration.  These are
low-level details -- users of parallel iterators should not need to
interact with them directly.  See [the `plumbing` README][r] for a general overview.

[r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md

```rust
pub mod plumbing { /* ... */ }
```

### Traits

#### Trait `ProducerCallback`

The `ProducerCallback` trait is a kind of generic closure,
[analogous to `FnOnce`][FnOnce]. See [the corresponding section in
the plumbing README][r] for more details.

[r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md#producer-callback
[FnOnce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html

```rust
pub trait ProducerCallback<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Output`: The type of value returned by this callback. Analogous to

###### Required Methods

- `callback`: Invokes the callback with the given producer as argument. The

#### Trait `Producer`

A `Producer` is effectively a "splittable `IntoIterator`". That
is, a producer is a value which can be converted into an iterator
at any time: at that point, it simply produces items on demand,
like any iterator. But what makes a `Producer` special is that,
*before* we convert to an iterator, we can also **split** it at a
particular point using the `split_at` method. This will yield up
two producers, one producing the items before that point, and one
producing the items after that point (these two producers can then
independently be split further, or be converted into iterators).
In Rayon, this splitting is used to divide between threads.
See [the `plumbing` README][r] for further details.

Note that each producer will always produce a fixed number of
items N. However, this number N is not queryable through the API;
the consumer is expected to track it.

NB. You might expect `Producer` to extend the `IntoIterator`
trait.  However, [rust-lang/rust#20671][20671] prevents us from
declaring the DoubleEndedIterator and ExactSizeIterator
constraints on a required IntoIterator trait, so we inline
IntoIterator here until that issue is fixed.

[r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
[20671]: https://github.com/rust-lang/rust/issues/20671

```rust
pub trait Producer: Send + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Item`: The type of item that will be produced by this producer once
- `IntoIter`: The type of iterator we will become.

###### Required Methods

- `into_iter`: Convert `self` into an iterator; at this point, no more parallel splits
- `split_at`: Split into two producers; one produces items `0..index`, the

##### Provided Methods

- ```rust
  fn min_len(self: &Self) -> usize { /* ... */ }
  ```
  The minimum number of items that we will process

- ```rust
  fn max_len(self: &Self) -> usize { /* ... */ }
  ```
  The maximum number of items that we will process

- ```rust
  fn fold_with<F>(self: Self, folder: F) -> F
where
    F: Folder<<Self as >::Item> { /* ... */ }
  ```
  Iterate the producer, feeding each element to `folder`, and

#### Trait `Consumer`

A consumer is effectively a [generalized "fold" operation][fold],
and in fact each consumer will eventually be converted into a
[`Folder`]. What makes a consumer special is that, like a
[`Producer`], it can be **split** into multiple consumers using
the `split_at` method. When a consumer is split, it produces two
consumers, as well as a **reducer**. The two consumers can be fed
items independently, and when they are done the reducer is used to
combine their two results into one. See [the `plumbing`
README][r] for further details.

[r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[`Folder`]: trait.Folder.html
[`Producer`]: trait.Producer.html

```rust
pub trait Consumer<Item>: Send + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Folder`: The type of folder that this consumer can be converted into.
- `Reducer`: The type of reducer that is produced if this consumer is split.
- `Result`: The type of result that this consumer will ultimately produce.

###### Required Methods

- `split_at`: Divide the consumer into two consumers, one processing items
- `into_folder`: Convert the consumer into a folder that can consume items
- `full`: Hint whether this `Consumer` would like to stop processing

#### Trait `Folder`

The `Folder` trait encapsulates [the standard fold
operation][fold].  It can be fed many items using the `consume`
method. At the end, once all items have been consumed, it can then
be converted (using `complete`) into a final value.

[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold

```rust
pub trait Folder<Item>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Result`: The type of result that will ultimately be produced by the folder.

###### Required Methods

- `consume`: Consume next item and return new sequential state.
- `complete`: Finish consuming items, produce final result.
- `full`: Hint whether this `Folder` would like to stop processing

##### Provided Methods

- ```rust
  fn consume_iter<I>(self: Self, iter: I) -> Self
where
    I: IntoIterator<Item = Item> { /* ... */ }
  ```
  Consume items from the iterator until full, and return new sequential state.

#### Trait `Reducer`

The reducer is the final step of a `Consumer` -- after a consumer
has been split into two parts, and each of those parts has been
fully processed, we are left with two results. The reducer is then
used to combine those two results into one. See [the `plumbing`
README][r] for further details.

[r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md

```rust
pub trait Reducer<Result> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `reduce`: Reduce two final results into one; this is executed after a

#### Trait `UnindexedConsumer`

A stateless consumer can be freely copied. These consumers can be
used like regular consumers, but they also support a
`split_off_left` method that does not take an index to split, but
simply splits at some arbitrary point (`for_each`, for example,
produces an unindexed consumer).

```rust
pub trait UnindexedConsumer<I>: Consumer<I> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `split_off_left`: Splits off a "left" consumer and returns it. The `self`
- `to_reducer`: Creates a reducer that can be used to combine the results from

#### Trait `UnindexedProducer`

A variant on `Producer` which does not know its exact length or
cannot represent it in a `usize`. These producers act like
ordinary producers except that they cannot be told to split at a
particular point. Instead, you just ask them to split 'somewhere'.

(In principle, `Producer` could extend this trait; however, it
does not because to do so would require producers to carry their
own length with them.)

```rust
pub trait UnindexedProducer: Send + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Item`: The type of item returned by this producer.

###### Required Methods

- `split`: Split midway into a new producer if possible, otherwise return `None`.
- `fold_with`: Iterate the producer, feeding each element to `folder`, and

### Functions

#### Function `bridge`

This helper function is used to "connect" a parallel iterator to a
consumer. It will convert the `par_iter` into a producer P and
then pull items from P and feed them to `consumer`, splitting and
creating parallel threads as needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
[`drive_unindexed`] or [`drive`] methods.

[`drive_unindexed`]: ../trait.ParallelIterator.html#tymethod.drive_unindexed
[`drive`]: ../trait.IndexedParallelIterator.html#tymethod.drive

```rust
pub fn bridge<I, C>(par_iter: I, consumer: C) -> <C as >::Result
where
    I: IndexedParallelIterator,
    C: Consumer<<I as >::Item> { /* ... */ }
```

#### Function `bridge_producer_consumer`

This helper function is used to "connect" a producer and a
consumer. You may prefer to call [`bridge`], which wraps this
function. This function will draw items from `producer` and feed
them to `consumer`, splitting and creating parallel tasks when
needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
[`drive_unindexed`] or [`drive`] methods.

[`bridge`]: fn.bridge.html
[`drive_unindexed`]: ../trait.ParallelIterator.html#tymethod.drive_unindexed
[`drive`]: ../trait.IndexedParallelIterator.html#tymethod.drive

```rust
pub fn bridge_producer_consumer<P, C>(len: usize, producer: P, consumer: C) -> <C as >::Result
where
    P: Producer,
    C: Consumer<<P as >::Item> { /* ... */ }
```

#### Function `bridge_unindexed`

A variant of [`bridge_producer_consumer`] where the producer is an unindexed producer.

[`bridge_producer_consumer`]: fn.bridge_producer_consumer.html

```rust
pub fn bridge_unindexed<P, C>(producer: P, consumer: C) -> <C as >::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<<P as >::Item> { /* ... */ }
```

### Traits

#### Trait `IntoParallelIterator`

`IntoParallelIterator` implements the conversion to a [`ParallelIterator`].

By implementing `IntoParallelIterator` for a type, you define how it will
transformed into an iterator. This is a parallel version of the standard
library's [`std::iter::IntoIterator`] trait.

[`ParallelIterator`]: trait.ParallelIterator.html
[`std::iter::IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

```rust
pub trait IntoParallelIterator {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: The parallel iterator type that will be created.
- `Item`: The type of item that the parallel iterator will produce.

###### Required Methods

- `into_par_iter`: Converts `self` into a parallel iterator.

##### Implementations

This trait is implemented for the following types:

- `&''data [T; N]` with <''data, T: Sync + ''data, const N: usize>
- `&''data mut [T; N]` with <''data, T: Send + ''data, const N: usize>
- `[T; N]` with <T: Send, const N: usize>
- `std::collections::BinaryHeap<T>` with <T: Ord + Send>
- `&''a std::collections::BinaryHeap<T>` with <''a, T: Ord + Sync>
- `std::collections::BTreeMap<K, V>` with <K: Ord + Send, V: Send>
- `&''a std::collections::BTreeMap<K, V>` with <''a, K: Ord + Sync, V: Sync>
- `&''a mut std::collections::BTreeMap<K, V>` with <''a, K: Ord + Sync, V: Send>
- `std::collections::BTreeSet<T>` with <T: Ord + Send>
- `&''a std::collections::BTreeSet<T>` with <''a, T: Ord + Sync>
- `std::collections::HashMap<K, V, S>` with <K: Hash + Eq + Send, V: Send, S: BuildHasher>
- `&''a std::collections::HashMap<K, V, S>` with <''a, K: Hash + Eq + Sync, V: Sync, S: BuildHasher>
- `&''a mut std::collections::HashMap<K, V, S>` with <''a, K: Hash + Eq + Sync, V: Send, S: BuildHasher>
- `std::collections::HashSet<T, S>` with <T: Hash + Eq + Send, S: BuildHasher>
- `&''a std::collections::HashSet<T, S>` with <''a, T: Hash + Eq + Sync, S: BuildHasher>
- `std::collections::LinkedList<T>` with <T: Send>
- `&''a std::collections::LinkedList<T>` with <''a, T: Sync>
- `&''a mut std::collections::LinkedList<T>` with <''a, T: Send>
- `std::collections::VecDeque<T>` with <T: Send>
- `&''a std::collections::VecDeque<T>` with <''a, T: Sync>
- `&''a mut std::collections::VecDeque<T>` with <''a, T: Send>
- `(A)` with <A>
- `&''a (A)` with <''a, A>
- `&''a mut (A)` with <''a, A>
- `(A, B)` with <A, B>
- `&''a (A, B)` with <''a, A, B>
- `&''a mut (A, B)` with <''a, A, B>
- `(A, B, C)` with <A, B, C>
- `&''a (A, B, C)` with <''a, A, B, C>
- `&''a mut (A, B, C)` with <''a, A, B, C>
- `(A, B, C, D)` with <A, B, C, D>
- `&''a (A, B, C, D)` with <''a, A, B, C, D>
- `&''a mut (A, B, C, D)` with <''a, A, B, C, D>
- `(A, B, C, D, E)` with <A, B, C, D, E>
- `&''a (A, B, C, D, E)` with <''a, A, B, C, D, E>
- `&''a mut (A, B, C, D, E)` with <''a, A, B, C, D, E>
- `(A, B, C, D, E, F)` with <A, B, C, D, E, F>
- `&''a (A, B, C, D, E, F)` with <''a, A, B, C, D, E, F>
- `&''a mut (A, B, C, D, E, F)` with <''a, A, B, C, D, E, F>
- `(A, B, C, D, E, F, G)` with <A, B, C, D, E, F, G>
- `&''a (A, B, C, D, E, F, G)` with <''a, A, B, C, D, E, F, G>
- `&''a mut (A, B, C, D, E, F, G)` with <''a, A, B, C, D, E, F, G>
- `(A, B, C, D, E, F, G, H)` with <A, B, C, D, E, F, G, H>
- `&''a (A, B, C, D, E, F, G, H)` with <''a, A, B, C, D, E, F, G, H>
- `&''a mut (A, B, C, D, E, F, G, H)` with <''a, A, B, C, D, E, F, G, H>
- `(A, B, C, D, E, F, G, H, I)` with <A, B, C, D, E, F, G, H, I>
- `&''a (A, B, C, D, E, F, G, H, I)` with <''a, A, B, C, D, E, F, G, H, I>
- `&''a mut (A, B, C, D, E, F, G, H, I)` with <''a, A, B, C, D, E, F, G, H, I>
- `(A, B, C, D, E, F, G, H, I, J)` with <A, B, C, D, E, F, G, H, I, J>
- `&''a (A, B, C, D, E, F, G, H, I, J)` with <''a, A, B, C, D, E, F, G, H, I, J>
- `&''a mut (A, B, C, D, E, F, G, H, I, J)` with <''a, A, B, C, D, E, F, G, H, I, J>
- `(A, B, C, D, E, F, G, H, I, J, K)` with <A, B, C, D, E, F, G, H, I, J, K>
- `&''a (A, B, C, D, E, F, G, H, I, J, K)` with <''a, A, B, C, D, E, F, G, H, I, J, K>
- `&''a mut (A, B, C, D, E, F, G, H, I, J, K)` with <''a, A, B, C, D, E, F, G, H, I, J, K>
- `(A, B, C, D, E, F, G, H, I, J, K, L)` with <A, B, C, D, E, F, G, H, I, J, K, L>
- `&''a (A, B, C, D, E, F, G, H, I, J, K, L)` with <''a, A, B, C, D, E, F, G, H, I, J, K, L>
- `&''a mut (A, B, C, D, E, F, G, H, I, J, K, L)` with <''a, A, B, C, D, E, F, G, H, I, J, K, L>
- `T` with <T: ParallelIterator>
- `Option<T>` with <T: Send>
- `&''a Option<T>` with <''a, T: Sync>
- `&''a mut Option<T>` with <''a, T: Send>
- `std::ops::Range<T>` with <T>
- `std::ops::RangeInclusive<T>` with <T>
- `Result<T, E>` with <T: Send, E>
- `&''a Result<T, E>` with <''a, T: Sync, E>
- `&''a mut Result<T, E>` with <''a, T: Send, E>
- `&''data [T]` with <''data, T: Sync + ''data>
- `&''data mut [T]` with <''data, T: Send + ''data>
- `&''data Vec<T>` with <''data, T: Sync + ''data>
- `&''data mut Vec<T>` with <''data, T: Send + ''data>
- `Vec<T>` with <T: Send>

#### Trait `IntoParallelRefIterator`

`IntoParallelRefIterator` implements the conversion to a
[`ParallelIterator`], providing shared references to the data.

This is a parallel version of the `iter()` method
defined by various collections.

This trait is automatically implemented
`for I where &I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`] rather than implement
this trait directly.

[`ParallelIterator`]: trait.ParallelIterator.html
[`IntoParallelIterator`]: trait.IntoParallelIterator.html

```rust
pub trait IntoParallelRefIterator<''data> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: The type of the parallel iterator that will be returned.
- `Item`: The type of item that the parallel iterator will produce.

###### Required Methods

- `par_iter`: Converts `self` into a parallel iterator.

##### Implementations

This trait is implemented for the following types:

- `I` with <''data, I: ''data + ?Sized>

#### Trait `IntoParallelRefMutIterator`

`IntoParallelRefMutIterator` implements the conversion to a
[`ParallelIterator`], providing mutable references to the data.

This is a parallel version of the `iter_mut()` method
defined by various collections.

This trait is automatically implemented
`for I where &mut I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`] rather than implement
this trait directly.

[`ParallelIterator`]: trait.ParallelIterator.html
[`IntoParallelIterator`]: trait.IntoParallelIterator.html

```rust
pub trait IntoParallelRefMutIterator<''data> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: The type of iterator that will be created.
- `Item`: The type of item that will be produced; this is typically an

###### Required Methods

- `par_iter_mut`: Creates the parallel iterator from `self`.

##### Implementations

This trait is implemented for the following types:

- `I` with <''data, I: ''data + ?Sized>

#### Trait `ParallelIterator`

Parallel version of the standard iterator trait.

The combinators on this trait are available on **all** parallel
iterators.  Additional methods can be found on the
[`IndexedParallelIterator`] trait: those methods are only
available for parallel iterators where the number of items is
known in advance (so, e.g., after invoking `filter`, those methods
become unavailable).

For examples of using parallel iterators, see [the docs on the
`iter` module][iter].

[iter]: index.html
[`IndexedParallelIterator`]: trait.IndexedParallelIterator.html

```rust
pub trait ParallelIterator: Sized + Send {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Item`: The type of item that this parallel iterator produces.

###### Required Methods

- `drive_unindexed`: Internal method used to define the behavior of this parallel

##### Provided Methods

- ```rust
  fn for_each<OP>(self: Self, op: OP)
where
    OP: Fn(<Self as >::Item) + Sync + Send { /* ... */ }
  ```
  Executes `OP` on each item produced by the iterator, in parallel.

- ```rust
  fn for_each_with<OP, T>(self: Self, init: T, op: OP)
where
    OP: Fn(&mut T, <Self as >::Item) + Sync + Send,
    T: Send + Clone { /* ... */ }
  ```
  Executes `OP` on the given `init` value with each item produced by

- ```rust
  fn for_each_init<OP, INIT, T>(self: Self, init: INIT, op: OP)
where
    OP: Fn(&mut T, <Self as >::Item) + Sync + Send,
    INIT: Fn() -> T + Sync + Send { /* ... */ }
  ```
  Executes `OP` on a value returned by `init` with each item produced by

- ```rust
  fn try_for_each<OP, R>(self: Self, op: OP) -> R
where
    OP: Fn(<Self as >::Item) -> R + Sync + Send,
    R: Try<Output = ()> + Send { /* ... */ }
  ```
  Executes a fallible `OP` on each item produced by the iterator, in parallel.

- ```rust
  fn try_for_each_with<OP, T, R>(self: Self, init: T, op: OP) -> R
where
    OP: Fn(&mut T, <Self as >::Item) -> R + Sync + Send,
    T: Send + Clone,
    R: Try<Output = ()> + Send { /* ... */ }
  ```
  Executes a fallible `OP` on the given `init` value with each item

- ```rust
  fn try_for_each_init<OP, INIT, T, R>(self: Self, init: INIT, op: OP) -> R
where
    OP: Fn(&mut T, <Self as >::Item) -> R + Sync + Send,
    INIT: Fn() -> T + Sync + Send,
    R: Try<Output = ()> + Send { /* ... */ }
  ```
  Executes a fallible `OP` on a value returned by `init` with each item

- ```rust
  fn count(self: Self) -> usize { /* ... */ }
  ```
  Counts the number of items in this parallel iterator.

- ```rust
  fn map<F, R>(self: Self, map_op: F) -> Map<Self, F>
where
    F: Fn(<Self as >::Item) -> R + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies `map_op` to each item of this iterator, producing a new

- ```rust
  fn map_with<F, T, R>(self: Self, init: T, map_op: F) -> MapWith<Self, T, F>
where
    F: Fn(&mut T, <Self as >::Item) -> R + Sync + Send,
    T: Send + Clone,
    R: Send { /* ... */ }
  ```
  Applies `map_op` to the given `init` value with each item of this

- ```rust
  fn map_init<F, INIT, T, R>(self: Self, init: INIT, map_op: F) -> MapInit<Self, INIT, F>
where
    F: Fn(&mut T, <Self as >::Item) -> R + Sync + Send,
    INIT: Fn() -> T + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies `map_op` to a value returned by `init` with each item of this

- ```rust
  fn cloned<''a, T>(self: Self) -> Cloned<Self>
where
    T: ''a + Clone + Send,
    Self: ParallelIterator<Item = &''a T> { /* ... */ }
  ```
  Creates an iterator which clones all of its elements.  This may be

- ```rust
  fn copied<''a, T>(self: Self) -> Copied<Self>
where
    T: ''a + Copy + Send,
    Self: ParallelIterator<Item = &''a T> { /* ... */ }
  ```
  Creates an iterator which copies all of its elements.  This may be

- ```rust
  fn inspect<OP>(self: Self, inspect_op: OP) -> Inspect<Self, OP>
where
    OP: Fn(&<Self as >::Item) + Sync + Send { /* ... */ }
  ```
  Applies `inspect_op` to a reference to each item of this iterator,

- ```rust
  fn update<F>(self: Self, update_op: F) -> Update<Self, F>
where
    F: Fn(&mut <Self as >::Item) + Sync + Send { /* ... */ }
  ```
  Mutates each item of this iterator before yielding it.

- ```rust
  fn filter<P>(self: Self, filter_op: P) -> Filter<Self, P>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Applies `filter_op` to each item of this iterator, producing a new

- ```rust
  fn filter_map<P, R>(self: Self, filter_op: P) -> FilterMap<Self, P>
where
    P: Fn(<Self as >::Item) -> Option<R> + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies `filter_op` to each item of this iterator to get an `Option`,

- ```rust
  fn flat_map<F, PI>(self: Self, map_op: F) -> FlatMap<Self, F>
where
    F: Fn(<Self as >::Item) -> PI + Sync + Send,
    PI: IntoParallelIterator { /* ... */ }
  ```
  Applies `map_op` to each item of this iterator to get nested parallel iterators,

- ```rust
  fn flat_map_iter<F, SI>(self: Self, map_op: F) -> FlatMapIter<Self, F>
where
    F: Fn(<Self as >::Item) -> SI + Sync + Send,
    SI: IntoIterator,
    <SI as >::Item: Send { /* ... */ }
  ```
  Applies `map_op` to each item of this iterator to get nested serial iterators,

- ```rust
  fn flatten(self: Self) -> Flatten<Self>
where
    <Self as >::Item: IntoParallelIterator { /* ... */ }
  ```
  An adaptor that flattens parallel-iterable `Item`s into one large iterator.

- ```rust
  fn flatten_iter(self: Self) -> FlattenIter<Self>
where
    <Self as >::Item: IntoIterator,
    <<Self as >::Item as IntoIterator>::Item: Send { /* ... */ }
  ```
  An adaptor that flattens serial-iterable `Item`s into one large iterator.

- ```rust
  fn reduce<OP, ID>(self: Self, identity: ID, op: OP) -> <Self as >::Item
where
    OP: Fn(<Self as >::Item, <Self as >::Item) -> <Self as >::Item + Sync + Send,
    ID: Fn() -> <Self as >::Item + Sync + Send { /* ... */ }
  ```
  Reduces the items in the iterator into one item using `op`.

- ```rust
  fn reduce_with<OP>(self: Self, op: OP) -> Option<<Self as >::Item>
where
    OP: Fn(<Self as >::Item, <Self as >::Item) -> <Self as >::Item + Sync + Send { /* ... */ }
  ```
  Reduces the items in the iterator into one item using `op`.

- ```rust
  fn try_reduce<T, OP, ID>(self: Self, identity: ID, op: OP) -> <Self as >::Item
where
    OP: Fn(T, T) -> <Self as >::Item + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    <Self as >::Item: Try<Output = T> { /* ... */ }
  ```
  Reduces the items in the iterator into one item using a fallible `op`.

- ```rust
  fn try_reduce_with<T, OP>(self: Self, op: OP) -> Option<<Self as >::Item>
where
    OP: Fn(T, T) -> <Self as >::Item + Sync + Send,
    <Self as >::Item: Try<Output = T> { /* ... */ }
  ```
  Reduces the items in the iterator into one item using a fallible `op`.

- ```rust
  fn fold<T, ID, F>(self: Self, identity: ID, fold_op: F) -> Fold<Self, ID, F>
where
    F: Fn(T, <Self as >::Item) -> T + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    T: Send { /* ... */ }
  ```
  Parallel fold is similar to sequential fold except that the

- ```rust
  fn fold_with<F, T>(self: Self, init: T, fold_op: F) -> FoldWith<Self, T, F>
where
    F: Fn(T, <Self as >::Item) -> T + Sync + Send,
    T: Send + Clone { /* ... */ }
  ```
  Applies `fold_op` to the given `init` value with each item of this

- ```rust
  fn try_fold<T, R, ID, F>(self: Self, identity: ID, fold_op: F) -> TryFold<Self, R, ID, F>
where
    F: Fn(T, <Self as >::Item) -> R + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    R: Try<Output = T> + Send { /* ... */ }
  ```
  Performs a fallible parallel fold.

- ```rust
  fn try_fold_with<F, T, R>(self: Self, init: T, fold_op: F) -> TryFoldWith<Self, R, F>
where
    F: Fn(T, <Self as >::Item) -> R + Sync + Send,
    R: Try<Output = T> + Send,
    T: Clone + Send { /* ... */ }
  ```
  Performs a fallible parallel fold with a cloneable `init` value.

- ```rust
  fn sum<S>(self: Self) -> S
where
    S: Send + Sum<<Self as >::Item> + Sum<S> { /* ... */ }
  ```
  Sums up the items in the iterator.

- ```rust
  fn product<P>(self: Self) -> P
where
    P: Send + Product<<Self as >::Item> + Product<P> { /* ... */ }
  ```
  Multiplies all the items in the iterator.

- ```rust
  fn min(self: Self) -> Option<<Self as >::Item>
where
    <Self as >::Item: Ord { /* ... */ }
  ```
  Computes the minimum of all the items in the iterator. If the

- ```rust
  fn min_by<F>(self: Self, f: F) -> Option<<Self as >::Item>
where
    F: Sync + Send + Fn(&<Self as >::Item, &<Self as >::Item) -> Ordering { /* ... */ }
  ```
  Computes the minimum of all the items in the iterator with respect to

- ```rust
  fn min_by_key<K, F>(self: Self, f: F) -> Option<<Self as >::Item>
where
    K: Ord + Send,
    F: Sync + Send + Fn(&<Self as >::Item) -> K { /* ... */ }
  ```
  Computes the item that yields the minimum value for the given

- ```rust
  fn max(self: Self) -> Option<<Self as >::Item>
where
    <Self as >::Item: Ord { /* ... */ }
  ```
  Computes the maximum of all the items in the iterator. If the

- ```rust
  fn max_by<F>(self: Self, f: F) -> Option<<Self as >::Item>
where
    F: Sync + Send + Fn(&<Self as >::Item, &<Self as >::Item) -> Ordering { /* ... */ }
  ```
  Computes the maximum of all the items in the iterator with respect to

- ```rust
  fn max_by_key<K, F>(self: Self, f: F) -> Option<<Self as >::Item>
where
    K: Ord + Send,
    F: Sync + Send + Fn(&<Self as >::Item) -> K { /* ... */ }
  ```
  Computes the item that yields the maximum value for the given

- ```rust
  fn chain<C>(self: Self, chain: C) -> Chain<Self, <C as >::Iter>
where
    C: IntoParallelIterator<Item = <Self as >::Item> { /* ... */ }
  ```
  Takes two iterators and creates a new iterator over both.

- ```rust
  fn find_any<P>(self: Self, predicate: P) -> Option<<Self as >::Item>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for **some** item in the parallel iterator that

- ```rust
  fn find_first<P>(self: Self, predicate: P) -> Option<<Self as >::Item>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for the sequentially **first** item in the parallel iterator

- ```rust
  fn find_last<P>(self: Self, predicate: P) -> Option<<Self as >::Item>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for the sequentially **last** item in the parallel iterator

- ```rust
  fn find_map_any<P, R>(self: Self, predicate: P) -> Option<R>
where
    P: Fn(<Self as >::Item) -> Option<R> + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies the given predicate to the items in the parallel iterator

- ```rust
  fn find_map_first<P, R>(self: Self, predicate: P) -> Option<R>
where
    P: Fn(<Self as >::Item) -> Option<R> + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies the given predicate to the items in the parallel iterator and

- ```rust
  fn find_map_last<P, R>(self: Self, predicate: P) -> Option<R>
where
    P: Fn(<Self as >::Item) -> Option<R> + Sync + Send,
    R: Send { /* ... */ }
  ```
  Applies the given predicate to the items in the parallel iterator and

- ```rust
  fn any<P>(self: Self, predicate: P) -> bool
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for **some** item in the parallel iterator that

- ```rust
  fn all<P>(self: Self, predicate: P) -> bool
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Tests that every item in the parallel iterator matches the given

- ```rust
  fn while_some<T>(self: Self) -> WhileSome<Self>
where
    Self: ParallelIterator<Item = Option<T>>,
    T: Send { /* ... */ }
  ```
  Creates an iterator over the `Some` items of this iterator, halting

- ```rust
  fn panic_fuse(self: Self) -> PanicFuse<Self> { /* ... */ }
  ```
  Wraps an iterator with a fuse in case of panics, to halt all threads

- ```rust
  fn collect<C>(self: Self) -> C
where
    C: FromParallelIterator<<Self as >::Item> { /* ... */ }
  ```
  Creates a fresh collection containing all the elements produced

- ```rust
  fn unzip<A, B, FromA, FromB>(self: Self) -> (FromA, FromB)
where
    Self: ParallelIterator<Item = (A, B)>,
    FromA: Default + Send + ParallelExtend<A>,
    FromB: Default + Send + ParallelExtend<B>,
    A: Send,
    B: Send { /* ... */ }
  ```
  Unzips the items of a parallel iterator into a pair of arbitrary

- ```rust
  fn partition<A, B, P>(self: Self, predicate: P) -> (A, B)
where
    A: Default + Send + ParallelExtend<<Self as >::Item>,
    B: Default + Send + ParallelExtend<<Self as >::Item>,
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Partitions the items of a parallel iterator into a pair of arbitrary

- ```rust
  fn partition_map<A, B, P, L, R>(self: Self, predicate: P) -> (A, B)
where
    A: Default + Send + ParallelExtend<L>,
    B: Default + Send + ParallelExtend<R>,
    P: Fn(<Self as >::Item) -> Either<L, R> + Sync + Send,
    L: Send,
    R: Send { /* ... */ }
  ```
  Partitions and maps the items of a parallel iterator into a pair of

- ```rust
  fn intersperse(self: Self, element: <Self as >::Item) -> Intersperse<Self>
where
    <Self as >::Item: Clone { /* ... */ }
  ```
  Intersperses clones of an element between items of this iterator.

- ```rust
  fn take_any(self: Self, n: usize) -> TakeAny<Self> { /* ... */ }
  ```
  Creates an iterator that yields `n` elements from *anywhere* in the original iterator.

- ```rust
  fn skip_any(self: Self, n: usize) -> SkipAny<Self> { /* ... */ }
  ```
  Creates an iterator that skips `n` elements from *anywhere* in the original iterator.

- ```rust
  fn take_any_while<P>(self: Self, predicate: P) -> TakeAnyWhile<Self, P>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Creates an iterator that takes elements from *anywhere* in the original iterator

- ```rust
  fn skip_any_while<P>(self: Self, predicate: P) -> SkipAnyWhile<Self, P>
where
    P: Fn(&<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Creates an iterator that skips elements from *anywhere* in the original iterator

- ```rust
  fn collect_vec_list(self: Self) -> LinkedList<Vec<<Self as >::Item>> { /* ... */ }
  ```
  Collects this iterator into a linked list of vectors.

- ```rust
  fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Internal method used to define the behavior of this parallel

##### Implementations

This trait is implemented for the following types:

- `IntoIter<T, N>` with <T: Send, const N: usize>
- `IntoIter<T>` with <T: Ord + Send>
- `Iter<''a, T>` with <''a, T: Ord + Sync + ''a>
- `Drain<''a, T>` with <''a, T: Ord + Send>
- `IntoIter<K, V>` with <K: Ord + Send, V: Send>
- `Iter<''a, K, V>` with <''a, K: Ord + Sync + ''a, V: Sync + ''a>
- `IterMut<''a, K, V>` with <''a, K: Ord + Sync + ''a, V: Send + ''a>
- `IntoIter<T>` with <T: Ord + Send>
- `Iter<''a, T>` with <''a, T: Ord + Sync + ''a>
- `IntoIter<K, V>` with <K: Hash + Eq + Send, V: Send>
- `Iter<''a, K, V>` with <''a, K: Hash + Eq + Sync + ''a, V: Sync + ''a>
- `IterMut<''a, K, V>` with <''a, K: Hash + Eq + Sync + ''a, V: Send + ''a>
- `Drain<''_, K, V>` with <K: Hash + Eq + Send, V: Send>
- `IntoIter<T>` with <T: Hash + Eq + Send>
- `Iter<''a, T>` with <''a, T: Hash + Eq + Sync + ''a>
- `Drain<''_, T>` with <T: Hash + Eq + Send>
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `Drain<''a, T>` with <''a, T: Send>
- `ExponentialBlocks<I>` with <I>
- `UniformBlocks<I>` with <I>
- `Chain<A, B>` with <A, B>
- `Chunks<I>` with <I>
- `Cloned<I>` with <''a, T, I>
- `Copied<I>` with <''a, T, I>
- `Empty<T>` with <T: Send>
- `Enumerate<I>` with <I>
- `Filter<I, P>` with <I, P>
- `FilterMap<I, P>` with <I, P, R>
- `FlatMap<I, F>` with <I, F, PI>
- `FlatMapIter<I, F>` with <I, F, SI>
- `Flatten<I>` with <I>
- `FlattenIter<I>` with <I>
- `Fold<I, ID, F>` with <U, I, ID, F>
- `FoldWith<I, U, F>` with <U, I, F>
- `FoldChunks<I, ID, F>` with <I, ID, U, F>
- `FoldChunksWith<I, U, F>` with <I, U, F>
- `Inspect<I, F>` with <I, F>
- `Interleave<I, J>` with <I, J>
- `InterleaveShortest<I, J>` with <I, J>
- `Intersperse<I>` with <I>
- `MinLen<I>` with <I>
- `MaxLen<I>` with <I>
- `Map<I, F>` with <I, F, R>
- `MapWith<I, T, F>` with <I, T, F, R>
- `MapInit<I, INIT, F>` with <I, INIT, T, F, R>
- `MultiZip<(A)>` with <A>
- `MultiZip<(A, B)>` with <A, B>
- `MultiZip<(A, B, C)>` with <A, B, C>
- `MultiZip<(A, B, C, D)>` with <A, B, C, D>
- `MultiZip<(A, B, C, D, E)>` with <A, B, C, D, E>
- `MultiZip<(A, B, C, D, E, F)>` with <A, B, C, D, E, F>
- `MultiZip<(A, B, C, D, E, F, G)>` with <A, B, C, D, E, F, G>
- `MultiZip<(A, B, C, D, E, F, G, H)>` with <A, B, C, D, E, F, G, H>
- `MultiZip<(A, B, C, D, E, F, G, H, I)>` with <A, B, C, D, E, F, G, H, I>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J)>` with <A, B, C, D, E, F, G, H, I, J>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J, K)>` with <A, B, C, D, E, F, G, H, I, J, K>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J, K, L)>` with <A, B, C, D, E, F, G, H, I, J, K, L>
- `Once<T>` with <T: Send>
- `PanicFuse<I>` with <I>
- `IterBridge<Iter>` with <Iter: Iterator + Send>
- `Positions<I, P>` with <I, P>
- `Repeat<T>` with <T>
- `RepeatN<T>` with <T>
- `Rev<I>` with <I>
- `Skip<I>` with <I>
- `SkipAny<I>` with <I>
- `SkipAnyWhile<I, P>` with <I, P>
- `Split<D, S>` with <D, S>
- `StepBy<I>` with <I>
- `Take<I>` with <I>
- `TakeAny<I>` with <I>
- `TakeAnyWhile<I, P>` with <I, P>
- `TryFold<I, U, ID, F>` with <U, I, ID, F>
- `TryFoldWith<I, U, F>` with <U, I, F>
- `Update<I, F>` with <I, F>
- `WalkTreePrefix<S, B>` with <S, B, I>
- `WalkTreePostfix<S, B>` with <S, B, I>
- `WalkTree<S, B>` with <S, B, I>
- `WhileSome<I>` with <I, T>
- `Zip<A, B>` with <A, B>
- `ZipEq<A, B>` with <A, B>
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `Iter<T>` with <T: RangeInteger>
- `Iter<char>`
- `Iter<T>` with <T: RangeInteger>
- `Iter<char>`
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `ChunkBy<''data, T, P>` with <''data, T, P>
- `ChunkByMut<''data, T, P>` with <''data, T, P>
- `Chunks<''data, T>` with <''data, T: Sync + ''data>
- `ChunksExact<''data, T>` with <''data, T: Sync + ''data>
- `ChunksMut<''data, T>` with <''data, T: Send + ''data>
- `ChunksExactMut<''data, T>` with <''data, T: Send + ''data>
- `RChunks<''data, T>` with <''data, T: Sync + ''data>
- `RChunksExact<''data, T>` with <''data, T: Sync + ''data>
- `RChunksMut<''data, T>` with <''data, T: Send + ''data>
- `RChunksExactMut<''data, T>` with <''data, T: Send + ''data>
- `Iter<''data, T>` with <''data, T: Sync + ''data>
- `Windows<''data, T>` with <''data, T: Sync + ''data>
- `IterMut<''data, T>` with <''data, T: Send + ''data>
- `Split<''data, T, P>` with <''data, T, P>
- `SplitInclusive<''data, T, P>` with <''data, T, P>
- `SplitMut<''data, T, P>` with <''data, T, P>
- `SplitInclusiveMut<''data, T, P>` with <''data, T, P>
- `Chars<''ch>` with <''ch>
- `CharIndices<''ch>` with <''ch>
- `Bytes<''ch>` with <''ch>
- `EncodeUtf16<''ch>` with <''ch>
- `Split<''ch, P>` with <''ch, P: Pattern>
- `SplitInclusive<''ch, P>` with <''ch, P: Pattern>
- `SplitTerminator<''ch, P>` with <''ch, P: Pattern>
- `Lines<''ch>` with <''ch>
- `SplitWhitespace<''ch>` with <''ch>
- `SplitAsciiWhitespace<''ch>` with <''ch>
- `Matches<''ch, P>` with <''ch, P: Pattern>
- `MatchIndices<''ch, P>` with <''ch, P: Pattern>
- `Drain<''a>` with <''a>
- `IntoIter<T>` with <T: Send>
- `Drain<''data, T>` with <''data, T: Send>
- `Either<L, R>` with <L, R>

#### Trait `IndexedParallelIterator`

**Attributes:**

- `#[allow(clippy::len_without_is_empty)]`

An iterator that supports "random access" to its data, meaning
that you can split it at arbitrary indices and draw data from
those points.

**Note:** Not implemented for `u64`, `i64`, `u128`, or `i128` ranges

```rust
pub trait IndexedParallelIterator: ParallelIterator {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `len`: Produces an exact count of how many items this iterator will
- `drive`: Internal method used to define the behavior of this parallel
- `with_producer`: Internal method used to define the behavior of this parallel

##### Provided Methods

- ```rust
  fn by_exponential_blocks(self: Self) -> ExponentialBlocks<Self> { /* ... */ }
  ```
  Divides an iterator into sequential blocks of exponentially-increasing size.

- ```rust
  fn by_uniform_blocks(self: Self, block_size: usize) -> UniformBlocks<Self> { /* ... */ }
  ```
  Divides an iterator into sequential blocks of the given size.

- ```rust
  fn collect_into_vec(self: Self, target: &mut Vec<<Self as >::Item>) { /* ... */ }
  ```
  Collects the results of the iterator into the specified

- ```rust
  fn unzip_into_vecs<A, B>(self: Self, left: &mut Vec<A>, right: &mut Vec<B>)
where
    Self: IndexedParallelIterator<Item = (A, B)>,
    A: Send,
    B: Send { /* ... */ }
  ```
  Unzips the results of the iterator into the specified

- ```rust
  fn zip<Z>(self: Self, zip_op: Z) -> Zip<Self, <Z as >::Iter>
where
    Z: IntoParallelIterator,
    <Z as >::Iter: IndexedParallelIterator { /* ... */ }
  ```
  Iterates over tuples `(A, B)`, where the items `A` are from

- ```rust
  fn zip_eq<Z>(self: Self, zip_op: Z) -> ZipEq<Self, <Z as >::Iter>
where
    Z: IntoParallelIterator,
    <Z as >::Iter: IndexedParallelIterator { /* ... */ }
  ```
  The same as `Zip`, but requires that both iterators have the same length.

- ```rust
  fn interleave<I>(self: Self, other: I) -> Interleave<Self, <I as >::Iter>
where
    I: IntoParallelIterator<Item = <Self as >::Item>,
    <I as >::Iter: IndexedParallelIterator<Item = <Self as >::Item> { /* ... */ }
  ```
  Interleaves elements of this iterator and the other given

- ```rust
  fn interleave_shortest<I>(self: Self, other: I) -> InterleaveShortest<Self, <I as >::Iter>
where
    I: IntoParallelIterator<Item = <Self as >::Item>,
    <I as >::Iter: IndexedParallelIterator<Item = <Self as >::Item> { /* ... */ }
  ```
  Interleaves elements of this iterator and the other given

- ```rust
  fn chunks(self: Self, chunk_size: usize) -> Chunks<Self> { /* ... */ }
  ```
  Splits an iterator up into fixed-size chunks.

- ```rust
  fn fold_chunks<T, ID, F>(self: Self, chunk_size: usize, identity: ID, fold_op: F) -> FoldChunks<Self, ID, F>
where
    ID: Fn() -> T + Send + Sync,
    F: Fn(T, <Self as >::Item) -> T + Send + Sync,
    T: Send { /* ... */ }
  ```
  Splits an iterator into fixed-size chunks, performing a sequential [`fold()`] on

- ```rust
  fn fold_chunks_with<T, F>(self: Self, chunk_size: usize, init: T, fold_op: F) -> FoldChunksWith<Self, T, F>
where
    T: Send + Clone,
    F: Fn(T, <Self as >::Item) -> T + Send + Sync { /* ... */ }
  ```
  Splits an iterator into fixed-size chunks, performing a sequential [`fold()`] on

- ```rust
  fn cmp<I>(self: Self, other: I) -> Ordering
where
    I: IntoParallelIterator<Item = <Self as >::Item>,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: Ord { /* ... */ }
  ```
  Lexicographically compares the elements of this `ParallelIterator` with those of

- ```rust
  fn partial_cmp<I>(self: Self, other: I) -> Option<Ordering>
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialOrd<<I as >::Item> { /* ... */ }
  ```
  Lexicographically compares the elements of this `ParallelIterator` with those of

- ```rust
  fn eq<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialEq<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn ne<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialEq<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn lt<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialOrd<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn le<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialOrd<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn gt<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialOrd<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn ge<I>(self: Self, other: I) -> bool
where
    I: IntoParallelIterator,
    <I as >::Iter: IndexedParallelIterator,
    <Self as >::Item: PartialOrd<<I as >::Item> { /* ... */ }
  ```
  Determines if the elements of this `ParallelIterator`

- ```rust
  fn enumerate(self: Self) -> Enumerate<Self> { /* ... */ }
  ```
  Yields an index along with each item.

- ```rust
  fn step_by(self: Self, step: usize) -> StepBy<Self> { /* ... */ }
  ```
   Creates an iterator that steps by the given amount

- ```rust
  fn skip(self: Self, n: usize) -> Skip<Self> { /* ... */ }
  ```
  Creates an iterator that skips the first `n` elements.

- ```rust
  fn take(self: Self, n: usize) -> Take<Self> { /* ... */ }
  ```
  Creates an iterator that yields the first `n` elements.

- ```rust
  fn position_any<P>(self: Self, predicate: P) -> Option<usize>
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for **some** item in the parallel iterator that

- ```rust
  fn position_first<P>(self: Self, predicate: P) -> Option<usize>
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for the sequentially **first** item in the parallel iterator

- ```rust
  fn position_last<P>(self: Self, predicate: P) -> Option<usize>
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for the sequentially **last** item in the parallel iterator

- ```rust
  fn positions<P>(self: Self, predicate: P) -> Positions<Self, P>
where
    P: Fn(<Self as >::Item) -> bool + Sync + Send { /* ... */ }
  ```
  Searches for items in the parallel iterator that match the given

- ```rust
  fn rev(self: Self) -> Rev<Self> { /* ... */ }
  ```
  Produces a new iterator with the elements of this iterator in

- ```rust
  fn with_min_len(self: Self, min: usize) -> MinLen<Self> { /* ... */ }
  ```
  Sets the minimum length of iterators desired to process in each

- ```rust
  fn with_max_len(self: Self, max: usize) -> MaxLen<Self> { /* ... */ }
  ```
  Sets the maximum length of iterators desired to process in each

##### Implementations

This trait is implemented for the following types:

- `IntoIter<T, N>` with <T: Send, const N: usize>
- `IntoIter<T>` with <T: Ord + Send>
- `Iter<''a, T>` with <''a, T: Ord + Sync + ''a>
- `Drain<''a, T>` with <''a, T: Ord + Send>
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `Drain<''a, T>` with <''a, T: Send>
- `Chain<A, B>` with <A, B>
- `Chunks<I>` with <I>
- `Cloned<I>` with <''a, T, I>
- `Copied<I>` with <''a, T, I>
- `Empty<T>` with <T: Send>
- `Enumerate<I>` with <I>
- `FoldChunks<I, ID, F>` with <I, ID, U, F>
- `FoldChunksWith<I, U, F>` with <I, U, F>
- `Inspect<I, F>` with <I, F>
- `Interleave<I, J>` with <I, J>
- `InterleaveShortest<I, J>` with <I, J>
- `Intersperse<I>` with <I>
- `MinLen<I>` with <I>
- `MaxLen<I>` with <I>
- `Map<I, F>` with <I, F, R>
- `MapWith<I, T, F>` with <I, T, F, R>
- `MapInit<I, INIT, F>` with <I, INIT, T, F, R>
- `MultiZip<(A)>` with <A>
- `MultiZip<(A, B)>` with <A, B>
- `MultiZip<(A, B, C)>` with <A, B, C>
- `MultiZip<(A, B, C, D)>` with <A, B, C, D>
- `MultiZip<(A, B, C, D, E)>` with <A, B, C, D, E>
- `MultiZip<(A, B, C, D, E, F)>` with <A, B, C, D, E, F>
- `MultiZip<(A, B, C, D, E, F, G)>` with <A, B, C, D, E, F, G>
- `MultiZip<(A, B, C, D, E, F, G, H)>` with <A, B, C, D, E, F, G, H>
- `MultiZip<(A, B, C, D, E, F, G, H, I)>` with <A, B, C, D, E, F, G, H, I>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J)>` with <A, B, C, D, E, F, G, H, I, J>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J, K)>` with <A, B, C, D, E, F, G, H, I, J, K>
- `MultiZip<(A, B, C, D, E, F, G, H, I, J, K, L)>` with <A, B, C, D, E, F, G, H, I, J, K, L>
- `Once<T>` with <T: Send>
- `PanicFuse<I>` with <I>
- `RepeatN<T>` with <T>
- `Rev<I>` with <I>
- `Skip<I>` with <I>
- `StepBy<I>` with <I>
- `Take<I>` with <I>
- `Update<I, F>` with <I, F>
- `Zip<A, B>` with <A, B>
- `ZipEq<A, B>` with <A, B>
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `Iter<T>` with <T: IndexedRangeInteger>
- `Iter<char>`
- `Iter<T>` with <T: IndexedRangeInteger>
- `Iter<char>`
- `IntoIter<T>` with <T: Send>
- `Iter<''a, T>` with <''a, T: Sync + ''a>
- `IterMut<''a, T>` with <''a, T: Send + ''a>
- `Chunks<''data, T>` with <''data, T: Sync + ''data>
- `ChunksExact<''data, T>` with <''data, T: Sync + ''data>
- `ChunksMut<''data, T>` with <''data, T: Send + ''data>
- `ChunksExactMut<''data, T>` with <''data, T: Send + ''data>
- `RChunks<''data, T>` with <''data, T: Sync + ''data>
- `RChunksExact<''data, T>` with <''data, T: Sync + ''data>
- `RChunksMut<''data, T>` with <''data, T: Send + ''data>
- `RChunksExactMut<''data, T>` with <''data, T: Send + ''data>
- `Iter<''data, T>` with <''data, T: Sync + ''data>
- `Windows<''data, T>` with <''data, T: Sync + ''data>
- `IterMut<''data, T>` with <''data, T: Send + ''data>
- `IntoIter<T>` with <T: Send>
- `Drain<''data, T>` with <''data, T: Send>
- `Either<L, R>` with <L, R>

#### Trait `FromParallelIterator`

`FromParallelIterator` implements the creation of a collection
from a [`ParallelIterator`]. By implementing
`FromParallelIterator` for a given type, you define how it will be
created from an iterator.

`FromParallelIterator` is used through [`ParallelIterator`]'s [`collect()`] method.

[`ParallelIterator`]: trait.ParallelIterator.html
[`collect()`]: trait.ParallelIterator.html#method.collect

# Examples

Implementing `FromParallelIterator` for your type:

```
use rayon::prelude::*;
use std::mem;

struct BlackHole {
    mass: usize,
}

impl<T: Send> FromParallelIterator<T> for BlackHole {
    fn from_par_iter<I>(par_iter: I) -> Self
        where I: IntoParallelIterator<Item = T>
    {
        let par_iter = par_iter.into_par_iter();
        BlackHole {
            mass: par_iter.count() * mem::size_of::<T>(),
        }
    }
}

let bh: BlackHole = (0i32..1000).into_par_iter().collect();
assert_eq!(bh.mass, 4000);
```

```rust
pub trait FromParallelIterator<T>
where
    T: Send {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_par_iter`: Creates an instance of the collection from the parallel iterator `par_iter`.

##### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>
- `Box<[T]>` with <T>
- `std::rc::Rc<[T]>` with <T>
- `std::sync::Arc<[T]>` with <T>
- `std::collections::VecDeque<T>` with <T>
- `std::collections::BinaryHeap<T>` with <T>
- `std::collections::LinkedList<T>` with <T>
- `std::collections::HashMap<K, V, S>` with <K, V, S>
- `std::collections::BTreeMap<K, V>` with <K, V>
- `std::collections::HashSet<V, S>` with <V, S>
- `std::collections::BTreeSet<V>` with <V>
- `String`
- `String` with <''a>
- `String` with <''a>
- `String`
- `String`
- `String` with <''a>
- `std::ffi::OsString` with <''a>
- `std::ffi::OsString`
- `std::ffi::OsString` with <''a>
- `std::borrow::Cow<''a, C>` with <''a, C, T>
- `()`
- `(FromA, FromB)` with <A, B, FromA, FromB>
- `(A, B)` with <L, R, A, B>
- `Option<C>` with <C, T>
- `Result<C, E>` with <C, T, E>

#### Trait `ParallelExtend`

`ParallelExtend` extends an existing collection with items from a [`ParallelIterator`].

[`ParallelIterator`]: trait.ParallelIterator.html

# Examples

Implementing `ParallelExtend` for your type:

```
use rayon::prelude::*;
use std::mem;

struct BlackHole {
    mass: usize,
}

impl<T: Send> ParallelExtend<T> for BlackHole {
    fn par_extend<I>(&mut self, par_iter: I)
        where I: IntoParallelIterator<Item = T>
    {
        let par_iter = par_iter.into_par_iter();
        self.mass += par_iter.count() * mem::size_of::<T>();
    }
}

let mut bh = BlackHole { mass: 0 };
bh.par_extend(0i32..1000);
assert_eq!(bh.mass, 4000);
bh.par_extend(0i64..10);
assert_eq!(bh.mass, 4080);
```

```rust
pub trait ParallelExtend<T>
where
    T: Send {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `par_extend`: Extends an instance of the collection with the elements drawn

##### Implementations

This trait is implemented for the following types:

- `std::collections::BinaryHeap<T>` with <T>
- `std::collections::BinaryHeap<T>` with <''a, T>
- `std::collections::BTreeMap<K, V>` with <K, V>
- `std::collections::BTreeMap<K, V>` with <''a, K, V>
- `std::collections::BTreeSet<T>` with <T>
- `std::collections::BTreeSet<T>` with <''a, T>
- `std::collections::HashMap<K, V, S>` with <K, V, S>
- `std::collections::HashMap<K, V, S>` with <''a, K, V, S>
- `std::collections::HashSet<T, S>` with <T, S>
- `std::collections::HashSet<T, S>` with <''a, T, S>
- `std::collections::LinkedList<T>` with <T>
- `std::collections::LinkedList<T>` with <''a, T>
- `std::ffi::OsString` with <''a>
- `std::ffi::OsString`
- `std::ffi::OsString` with <''a>
- `String`
- `String` with <''a>
- `String` with <''a>
- `String`
- `String`
- `String` with <''a>
- `std::collections::VecDeque<T>` with <T>
- `std::collections::VecDeque<T>` with <''a, T>
- `Vec<T>` with <T>
- `Vec<T>` with <''a, T>
- `()`
- `(FromA, FromB)` with <A, B, FromA, FromB>
- `(A, B)` with <L, R, A, B>
- `Either<L, R>` with <L, R, T>

#### Trait `ParallelDrainFull`

`ParallelDrainFull` creates a parallel iterator that moves all items
from a collection while retaining the original capacity.

Types which are indexable typically implement [`ParallelDrainRange`]
instead, where you can drain fully with `par_drain(..)`.

[`ParallelDrainRange`]: trait.ParallelDrainRange.html

```rust
pub trait ParallelDrainFull {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: The draining parallel iterator type that will be created.
- `Item`: The type of item that the parallel iterator will produce.

###### Required Methods

- `par_drain`: Returns a draining parallel iterator over an entire collection.

##### Implementations

This trait is implemented for the following types:

- `&''a mut std::collections::BinaryHeap<T>` with <''a, T: Ord + Send>
- `&''a mut std::collections::HashMap<K, V, S>` with <''a, K: Hash + Eq + Send, V: Send, S: BuildHasher>
- `&''a mut std::collections::HashSet<T, S>` with <''a, T: Hash + Eq + Send, S: BuildHasher>

#### Trait `ParallelDrainRange`

`ParallelDrainRange` creates a parallel iterator that moves a range of items
from a collection while retaining the original capacity.

Types which are not indexable may implement [`ParallelDrainFull`] instead.

[`ParallelDrainFull`]: trait.ParallelDrainFull.html

```rust
pub trait ParallelDrainRange<Idx = usize> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Iter`: The draining parallel iterator type that will be created.
- `Item`: The type of item that the parallel iterator will produce.

###### Required Methods

- `par_drain`: Returns a draining parallel iterator over a range of the collection.

##### Implementations

This trait is implemented for the following types:

- `&''a mut std::collections::VecDeque<T>` with <''a, T: Send>
- `&''a mut String` with <''a>
- `&''data mut Vec<T>` with <''data, T: Send>

### Re-exports

#### Re-export `Either`

```rust
pub use either::Either;
```

#### Re-export `ExponentialBlocks`

```rust
pub use self::blocks::ExponentialBlocks;
```

#### Re-export `UniformBlocks`

```rust
pub use self::blocks::UniformBlocks;
```

#### Re-export `Chain`

```rust
pub use self::chain::Chain;
```

#### Re-export `Chunks`

```rust
pub use self::chunks::Chunks;
```

#### Re-export `Cloned`

```rust
pub use self::cloned::Cloned;
```

#### Re-export `Copied`

```rust
pub use self::copied::Copied;
```

#### Re-export `empty`

```rust
pub use self::empty::empty;
```

#### Re-export `Empty`

```rust
pub use self::empty::Empty;
```

#### Re-export `Enumerate`

```rust
pub use self::enumerate::Enumerate;
```

#### Re-export `Filter`

```rust
pub use self::filter::Filter;
```

#### Re-export `FilterMap`

```rust
pub use self::filter_map::FilterMap;
```

#### Re-export `FlatMap`

```rust
pub use self::flat_map::FlatMap;
```

#### Re-export `FlatMapIter`

```rust
pub use self::flat_map_iter::FlatMapIter;
```

#### Re-export `Flatten`

```rust
pub use self::flatten::Flatten;
```

#### Re-export `FlattenIter`

```rust
pub use self::flatten_iter::FlattenIter;
```

#### Re-export `Fold`

```rust
pub use self::fold::Fold;
```

#### Re-export `FoldWith`

```rust
pub use self::fold::FoldWith;
```

#### Re-export `FoldChunks`

```rust
pub use self::fold_chunks::FoldChunks;
```

#### Re-export `FoldChunksWith`

```rust
pub use self::fold_chunks_with::FoldChunksWith;
```

#### Re-export `Inspect`

```rust
pub use self::inspect::Inspect;
```

#### Re-export `Interleave`

```rust
pub use self::interleave::Interleave;
```

#### Re-export `InterleaveShortest`

```rust
pub use self::interleave_shortest::InterleaveShortest;
```

#### Re-export `Intersperse`

```rust
pub use self::intersperse::Intersperse;
```

#### Re-export `MaxLen`

```rust
pub use self::len::MaxLen;
```

#### Re-export `MinLen`

```rust
pub use self::len::MinLen;
```

#### Re-export `Map`

```rust
pub use self::map::Map;
```

#### Re-export `MapInit`

```rust
pub use self::map_with::MapInit;
```

#### Re-export `MapWith`

```rust
pub use self::map_with::MapWith;
```

#### Re-export `MultiZip`

```rust
pub use self::multizip::MultiZip;
```

#### Re-export `once`

```rust
pub use self::once::once;
```

#### Re-export `Once`

```rust
pub use self::once::Once;
```

#### Re-export `PanicFuse`

```rust
pub use self::panic_fuse::PanicFuse;
```

#### Re-export `IterBridge`

```rust
pub use self::par_bridge::IterBridge;
```

#### Re-export `ParallelBridge`

```rust
pub use self::par_bridge::ParallelBridge;
```

#### Re-export `Positions`

```rust
pub use self::positions::Positions;
```

#### Re-export `repeat`

```rust
pub use self::repeat::repeat;
```

#### Re-export `repeatn`

```rust
pub use self::repeat::repeatn;
```

#### Re-export `Repeat`

```rust
pub use self::repeat::Repeat;
```

#### Re-export `RepeatN`

```rust
pub use self::repeat::RepeatN;
```

#### Re-export `Rev`

```rust
pub use self::rev::Rev;
```

#### Re-export `Skip`

```rust
pub use self::skip::Skip;
```

#### Re-export `SkipAny`

```rust
pub use self::skip_any::SkipAny;
```

#### Re-export `SkipAnyWhile`

```rust
pub use self::skip_any_while::SkipAnyWhile;
```

#### Re-export `split`

```rust
pub use self::splitter::split;
```

#### Re-export `Split`

```rust
pub use self::splitter::Split;
```

#### Re-export `StepBy`

```rust
pub use self::step_by::StepBy;
```

#### Re-export `Take`

```rust
pub use self::take::Take;
```

#### Re-export `TakeAny`

```rust
pub use self::take_any::TakeAny;
```

#### Re-export `TakeAnyWhile`

```rust
pub use self::take_any_while::TakeAnyWhile;
```

#### Re-export `TryFold`

```rust
pub use self::try_fold::TryFold;
```

#### Re-export `TryFoldWith`

```rust
pub use self::try_fold::TryFoldWith;
```

#### Re-export `Update`

```rust
pub use self::update::Update;
```

#### Re-export `walk_tree`

```rust
pub use self::walk_tree::walk_tree;
```

#### Re-export `walk_tree_postfix`

```rust
pub use self::walk_tree::walk_tree_postfix;
```

#### Re-export `walk_tree_prefix`

```rust
pub use self::walk_tree::walk_tree_prefix;
```

#### Re-export `WalkTree`

```rust
pub use self::walk_tree::WalkTree;
```

#### Re-export `WalkTreePostfix`

```rust
pub use self::walk_tree::WalkTreePostfix;
```

#### Re-export `WalkTreePrefix`

```rust
pub use self::walk_tree::WalkTreePrefix;
```

#### Re-export `WhileSome`

```rust
pub use self::while_some::WhileSome;
```

#### Re-export `Zip`

```rust
pub use self::zip::Zip;
```

#### Re-export `ZipEq`

```rust
pub use self::zip_eq::ZipEq;
```

## Module `option`

Parallel iterator types for [options][std::option]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[std::option]: https://doc.rust-lang.org/stable/std/option/

```rust
pub mod option { /* ... */ }
```

### Types

#### Struct `IntoIter`

A parallel iterator over the value in [`Some`] variant of an [`Option`].

The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.

This `struct` is created by the [`into_par_iter`] function.

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
[`into_par_iter`]: ../iter/trait.IntoParallelIterator.html#tymethod.into_par_iter

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Iter`

A parallel iterator over a reference to the [`Some`] variant of an [`Option`].

The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.

This `struct` is created by the [`par_iter`] function.

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
[`par_iter`]: ../iter/trait.IntoParallelRefIterator.html#tymethod.par_iter

```rust
pub struct Iter<''a, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Freeze**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IterMut`

A parallel iterator over a mutable reference to the [`Some`] variant of an [`Option`].

The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.

This `struct` is created by the [`par_iter_mut`] function.

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
[`par_iter_mut`]: ../iter/trait.IntoParallelRefMutIterator.html#tymethod.par_iter_mut

```rust
pub struct IterMut<''a, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Sync**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `prelude`

The rayon prelude imports the various `ParallelIterator` traits.
The intention is that one can include `use rayon::prelude::*` and
have easy access to the various traits and methods you will need.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `FromParallelIterator`

```rust
pub use crate::iter::FromParallelIterator;
```

#### Re-export `IndexedParallelIterator`

```rust
pub use crate::iter::IndexedParallelIterator;
```

#### Re-export `IntoParallelIterator`

```rust
pub use crate::iter::IntoParallelIterator;
```

#### Re-export `IntoParallelRefIterator`

```rust
pub use crate::iter::IntoParallelRefIterator;
```

#### Re-export `IntoParallelRefMutIterator`

```rust
pub use crate::iter::IntoParallelRefMutIterator;
```

#### Re-export `ParallelBridge`

```rust
pub use crate::iter::ParallelBridge;
```

#### Re-export `ParallelDrainFull`

```rust
pub use crate::iter::ParallelDrainFull;
```

#### Re-export `ParallelDrainRange`

```rust
pub use crate::iter::ParallelDrainRange;
```

#### Re-export `ParallelExtend`

```rust
pub use crate::iter::ParallelExtend;
```

#### Re-export `ParallelIterator`

```rust
pub use crate::iter::ParallelIterator;
```

#### Re-export `ParallelSlice`

```rust
pub use crate::slice::ParallelSlice;
```

#### Re-export `ParallelSliceMut`

```rust
pub use crate::slice::ParallelSliceMut;
```

#### Re-export `ParallelString`

```rust
pub use crate::str::ParallelString;
```

## Module `range`

Parallel iterator types for [ranges][std::range],
the type for values created by `a..b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```
use rayon::prelude::*;

let r = (0..100u64).into_par_iter()
                   .sum();

// compare result with sequential calculation
assert_eq!((0..100).sum::<u64>(), r);
```

[std::range]: https://doc.rust-lang.org/core/ops/struct.Range.html

```rust
pub mod range { /* ... */ }
```

### Types

#### Struct `Iter`

Parallel iterator over a range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is not implemented for `u64`, `i64`, `u128`, or `i128`.

```
use rayon::prelude::*;

let p = (0..25usize).into_par_iter()
                  .zip(0..25usize)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<usize>();

let s = (0..25usize).zip(0..25)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

```rust
pub struct Iter<T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<T> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<T> { /* ... */ }
    ```

  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<T> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `range_inclusive`

Parallel iterator types for [inclusive ranges][std::range],
the type for values created by `a..=b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```
use rayon::prelude::*;

let r = (0..=100u64).into_par_iter()
                    .sum();

// compare result with sequential calculation
assert_eq!((0..=100).sum::<u64>(), r);
```

[std::range]: https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html

```rust
pub mod range_inclusive { /* ... */ }
```

### Types

#### Struct `Iter`

Parallel iterator over an inclusive range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is only implemented for `u8`, `i8`, `u16`, `i16`, and `char`.

```
use rayon::prelude::*;

let p = (0..=25u16).into_par_iter()
                  .zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<u16>();

let s = (0..=25u16).zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

```rust
pub struct Iter<T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<T> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<T> { /* ... */ }
    ```

  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<T> { /* ... */ }
    ```

- **Send**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<T> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `result`

Parallel iterator types for [results][std::result]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[std::result]: https://doc.rust-lang.org/stable/std/result/

```rust
pub mod result { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator over a result

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
#### Struct `Iter`

Parallel iterator over an immutable reference to a result

```rust
pub struct Iter<''a, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Send**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

#### Struct `IterMut`

Parallel iterator over a mutable reference to a result

```rust
pub struct IterMut<''a, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
## Module `slice`

Parallel iterator types for [slices][std::slice]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[std::slice]: https://doc.rust-lang.org/stable/std/slice/

```rust
pub mod slice { /* ... */ }
```

### Types

#### Struct `Iter`

Parallel iterator over immutable items in a slice

```rust
pub struct Iter<''data, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Freeze**
- **Send**
- **IntoEither**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Windows`

Parallel iterator over immutable overlapping windows of a slice

```rust
pub struct Windows<''data, T: Sync> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

#### Struct `IterMut`

Parallel iterator over mutable items in a slice

```rust
pub struct IterMut<''data, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

#### Struct `Split`

Parallel iterator over slices separated by a predicate

```rust
pub struct Split<''data, T, P> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **UnwindSafe**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

#### Struct `SplitInclusive`

Parallel iterator over slices separated by a predicate,
including the matched part as a terminator.

```rust
pub struct SplitInclusive<''data, T, P> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **IntoEither**
#### Struct `SplitMut`

Parallel iterator over mutable slices separated by a predicate

```rust
pub struct SplitMut<''data, T, P> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
#### Struct `SplitInclusiveMut`

Parallel iterator over mutable slices separated by a predicate,
including the matched part as a terminator.

```rust
pub struct SplitInclusiveMut<''data, T, P> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `ParallelSlice`

Parallel extensions for slices.

```rust
pub trait ParallelSlice<T: Sync> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_parallel_slice`: Returns a plain slice, which is used to implement the rest of the

##### Provided Methods

- ```rust
  fn par_split<P>(self: &Self, separator: P) -> Split<''_, T, P>
where
    P: Fn(&T) -> bool + Sync + Send { /* ... */ }
  ```
  Returns a parallel iterator over subslices separated by elements that

- ```rust
  fn par_split_inclusive<P>(self: &Self, separator: P) -> SplitInclusive<''_, T, P>
where
    P: Fn(&T) -> bool + Sync + Send { /* ... */ }
  ```
  Returns a parallel iterator over subslices separated by elements that

- ```rust
  fn par_windows(self: &Self, window_size: usize) -> Windows<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over all contiguous windows of length

- ```rust
  fn par_chunks(self: &Self, chunk_size: usize) -> Chunks<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over at most `chunk_size` elements of

- ```rust
  fn par_chunks_exact(self: &Self, chunk_size: usize) -> ChunksExact<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over `chunk_size` elements of

- ```rust
  fn par_rchunks(self: &Self, chunk_size: usize) -> RChunks<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- ```rust
  fn par_rchunks_exact(self: &Self, chunk_size: usize) -> RChunksExact<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- ```rust
  fn par_chunk_by<F>(self: &Self, pred: F) -> ChunkBy<''_, T, F>
where
    F: Fn(&T, &T) -> bool + Send + Sync { /* ... */ }
  ```
  Returns a parallel iterator over the slice producing non-overlapping runs

##### Implementations

This trait is implemented for the following types:

- `[T]` with <T: Sync>

#### Trait `ParallelSliceMut`

Parallel extensions for mutable slices.

```rust
pub trait ParallelSliceMut<T: Send> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_parallel_slice_mut`: Returns a plain mutable slice, which is used to implement the rest of

##### Provided Methods

- ```rust
  fn par_split_mut<P>(self: &mut Self, separator: P) -> SplitMut<''_, T, P>
where
    P: Fn(&T) -> bool + Sync + Send { /* ... */ }
  ```
  Returns a parallel iterator over mutable subslices separated by

- ```rust
  fn par_split_inclusive_mut<P>(self: &mut Self, separator: P) -> SplitInclusiveMut<''_, T, P>
where
    P: Fn(&T) -> bool + Sync + Send { /* ... */ }
  ```
  Returns a parallel iterator over mutable subslices separated by elements

- ```rust
  fn par_chunks_mut(self: &mut Self, chunk_size: usize) -> ChunksMut<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over at most `chunk_size` elements of

- ```rust
  fn par_chunks_exact_mut(self: &mut Self, chunk_size: usize) -> ChunksExactMut<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over `chunk_size` elements of

- ```rust
  fn par_rchunks_mut(self: &mut Self, chunk_size: usize) -> RChunksMut<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- ```rust
  fn par_rchunks_exact_mut(self: &mut Self, chunk_size: usize) -> RChunksExactMut<''_, T> { /* ... */ }
  ```
  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- ```rust
  fn par_sort(self: &mut Self)
where
    T: Ord { /* ... */ }
  ```
  Sorts the slice in parallel.

- ```rust
  fn par_sort_by<F>(self: &mut Self, compare: F)
where
    F: Fn(&T, &T) -> Ordering + Sync { /* ... */ }
  ```
  Sorts the slice in parallel with a comparator function.

- ```rust
  fn par_sort_by_key<K, F>(self: &mut Self, f: F)
where
    K: Ord,
    F: Fn(&T) -> K + Sync { /* ... */ }
  ```
  Sorts the slice in parallel with a key extraction function.

- ```rust
  fn par_sort_by_cached_key<K, F>(self: &mut Self, f: F)
where
    F: Fn(&T) -> K + Sync,
    K: Ord + Send { /* ... */ }
  ```
  Sorts the slice in parallel with a key extraction function.

- ```rust
  fn par_sort_unstable(self: &mut Self)
where
    T: Ord { /* ... */ }
  ```
  Sorts the slice in parallel, but might not preserve the order of equal elements.

- ```rust
  fn par_sort_unstable_by<F>(self: &mut Self, compare: F)
where
    F: Fn(&T, &T) -> Ordering + Sync { /* ... */ }
  ```
  Sorts the slice in parallel with a comparator function, but might not preserve the order of

- ```rust
  fn par_sort_unstable_by_key<K, F>(self: &mut Self, f: F)
where
    K: Ord,
    F: Fn(&T) -> K + Sync { /* ... */ }
  ```
  Sorts the slice in parallel with a key extraction function, but might not preserve the order

- ```rust
  fn par_chunk_by_mut<F>(self: &mut Self, pred: F) -> ChunkByMut<''_, T, F>
where
    F: Fn(&T, &T) -> bool + Send + Sync { /* ... */ }
  ```
  Returns a parallel iterator over the slice producing non-overlapping mutable

##### Implementations

This trait is implemented for the following types:

- `[T]` with <T: Send>

### Re-exports

#### Re-export `ChunkBy`

```rust
pub use self::chunk_by::ChunkBy;
```

#### Re-export `ChunkByMut`

```rust
pub use self::chunk_by::ChunkByMut;
```

#### Re-export `Chunks`

```rust
pub use self::chunks::Chunks;
```

#### Re-export `ChunksExact`

```rust
pub use self::chunks::ChunksExact;
```

#### Re-export `ChunksExactMut`

```rust
pub use self::chunks::ChunksExactMut;
```

#### Re-export `ChunksMut`

```rust
pub use self::chunks::ChunksMut;
```

#### Re-export `RChunks`

```rust
pub use self::rchunks::RChunks;
```

#### Re-export `RChunksExact`

```rust
pub use self::rchunks::RChunksExact;
```

#### Re-export `RChunksExactMut`

```rust
pub use self::rchunks::RChunksExactMut;
```

#### Re-export `RChunksMut`

```rust
pub use self::rchunks::RChunksMut;
```

## Module `str`

Parallel iterator types for [strings][std::str]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

Note: [`ParallelString::par_split()`] and [`par_split_terminator()`]
reference a `Pattern` trait which is not visible outside this crate.
This trait is intentionally kept private, for use only by Rayon itself.
It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
and any function or closure `F: Fn(char) -> bool + Sync + Send`.

[`ParallelString::par_split()`]: trait.ParallelString.html#method.par_split
[`par_split_terminator()`]: trait.ParallelString.html#method.par_split_terminator

[std::str]: https://doc.rust-lang.org/stable/std/str/

```rust
pub mod str { /* ... */ }
```

### Types

#### Struct `Chars`

Parallel iterator over the characters of a string

```rust
pub struct Chars<''ch> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Chars<''ch> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `CharIndices`

Parallel iterator over the characters of a string, with their positions

```rust
pub struct CharIndices<''ch> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CharIndices<''ch> { /* ... */ }
    ```

- **Sync**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Bytes`

Parallel iterator over the bytes of a string

```rust
pub struct Bytes<''ch> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Bytes<''ch> { /* ... */ }
    ```

#### Struct `EncodeUtf16`

Parallel iterator over a string encoded as UTF-16

```rust
pub struct EncodeUtf16<''ch> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EncodeUtf16<''ch> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `Split`

Parallel iterator over substrings separated by a pattern

```rust
pub struct Split<''ch, P: Pattern> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Split<''ch, P> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

#### Struct `SplitInclusive`

Parallel iterator over substrings separated by a pattern

```rust
pub struct SplitInclusive<''ch, P: Pattern> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SplitInclusive<''ch, P> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `SplitTerminator`

Parallel iterator over substrings separated by a terminator pattern

```rust
pub struct SplitTerminator<''ch, P: Pattern> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SplitTerminator<''ch, P> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
#### Struct `Lines`

Parallel iterator over lines in a string

```rust
pub struct Lines<''ch>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Lines<''ch> { /* ... */ }
    ```

- **IntoEither**
#### Struct `SplitWhitespace`

Parallel iterator over substrings separated by whitespace

```rust
pub struct SplitWhitespace<''ch>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SplitWhitespace<''ch> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `SplitAsciiWhitespace`

Parallel iterator over substrings separated by ASCII whitespace

```rust
pub struct SplitAsciiWhitespace<''ch>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SplitAsciiWhitespace<''ch> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Matches`

Parallel iterator over substrings that match a pattern

```rust
pub struct Matches<''ch, P: Pattern> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Matches<''ch, P> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
#### Struct `MatchIndices`

Parallel iterator over substrings that match a pattern, with their positions

```rust
pub struct MatchIndices<''ch, P: Pattern> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MatchIndices<''ch, P> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `ParallelString`

Parallel extensions for strings.

```rust
pub trait ParallelString {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_parallel_string`: Returns a plain string slice, which is used to implement the rest of

##### Provided Methods

- ```rust
  fn par_chars(self: &Self) -> Chars<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the characters of a string.

- ```rust
  fn par_char_indices(self: &Self) -> CharIndices<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the characters of a string, with their positions.

- ```rust
  fn par_bytes(self: &Self) -> Bytes<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the bytes of a string.

- ```rust
  fn par_encode_utf16(self: &Self) -> EncodeUtf16<''_> { /* ... */ }
  ```
  Returns a parallel iterator over a string encoded as UTF-16.

- ```rust
  fn par_split<P: Pattern>(self: &Self, separator: P) -> Split<''_, P> { /* ... */ }
  ```
  Returns a parallel iterator over substrings separated by a

- ```rust
  fn par_split_inclusive<P: Pattern>(self: &Self, separator: P) -> SplitInclusive<''_, P> { /* ... */ }
  ```
  Returns a parallel iterator over substrings separated by a

- ```rust
  fn par_split_terminator<P: Pattern>(self: &Self, terminator: P) -> SplitTerminator<''_, P> { /* ... */ }
  ```
  Returns a parallel iterator over substrings terminated by a

- ```rust
  fn par_lines(self: &Self) -> Lines<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the lines of a string, ending with an

- ```rust
  fn par_split_whitespace(self: &Self) -> SplitWhitespace<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the sub-slices of a string that are

- ```rust
  fn par_split_ascii_whitespace(self: &Self) -> SplitAsciiWhitespace<''_> { /* ... */ }
  ```
  Returns a parallel iterator over the sub-slices of a string that are

- ```rust
  fn par_matches<P: Pattern>(self: &Self, pattern: P) -> Matches<''_, P> { /* ... */ }
  ```
  Returns a parallel iterator over substrings that match a

- ```rust
  fn par_match_indices<P: Pattern>(self: &Self, pattern: P) -> MatchIndices<''_, P> { /* ... */ }
  ```
  Returns a parallel iterator over substrings that match a given character

##### Implementations

This trait is implemented for the following types:

- `str`

## Module `string`

This module contains the parallel iterator types for owned strings
(`String`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

```rust
pub mod string { /* ... */ }
```

### Types

#### Struct `Drain`

Draining parallel iterator that moves a range of characters out of a string,
but keeps the total capacity.

```rust
pub struct Drain<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

## Module `vec`

Parallel iterator types for [vectors][std::vec] (`Vec<T>`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[std::vec]: https://doc.rust-lang.org/stable/std/vec/

```rust
pub mod vec { /* ... */ }
```

### Types

#### Struct `IntoIter`

Parallel iterator that moves out of a vector.

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

#### Struct `Drain`

Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.

```rust
pub struct Drain<''data, T: Send> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **IntoEither**
- **RefUnwindSafe**
- **ParallelIterator**
  - ```rust
    fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: UnindexedConsumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn opt_len(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoParallelIterator**
  - ```rust
    fn into_par_iter(self: Self) -> T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IndexedParallelIterator**
  - ```rust
    fn drive<C>(self: Self, consumer: C) -> <C as >::Result
where
    C: Consumer<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output
where
    CB: ProducerCallback<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Re-exports

### Re-export `FnContext`

```rust
pub use rayon_core::FnContext;
```

### Re-export `ThreadBuilder`

```rust
pub use rayon_core::ThreadBuilder;
```

### Re-export `ThreadPool`

```rust
pub use rayon_core::ThreadPool;
```

### Re-export `ThreadPoolBuildError`

```rust
pub use rayon_core::ThreadPoolBuildError;
```

### Re-export `ThreadPoolBuilder`

```rust
pub use rayon_core::ThreadPoolBuilder;
```

### Re-export `broadcast`

```rust
pub use rayon_core::broadcast;
```

### Re-export `spawn_broadcast`

```rust
pub use rayon_core::spawn_broadcast;
```

### Re-export `BroadcastContext`

```rust
pub use rayon_core::BroadcastContext;
```

### Re-export `current_num_threads`

```rust
pub use rayon_core::current_num_threads;
```

### Re-export `current_thread_index`

```rust
pub use rayon_core::current_thread_index;
```

### Re-export `max_num_threads`

```rust
pub use rayon_core::max_num_threads;
```

### Re-export `in_place_scope`

```rust
pub use rayon_core::in_place_scope;
```

### Re-export `scope`

```rust
pub use rayon_core::scope;
```

### Re-export `Scope`

```rust
pub use rayon_core::Scope;
```

### Re-export `in_place_scope_fifo`

```rust
pub use rayon_core::in_place_scope_fifo;
```

### Re-export `scope_fifo`

```rust
pub use rayon_core::scope_fifo;
```

### Re-export `ScopeFifo`

```rust
pub use rayon_core::ScopeFifo;
```

### Re-export `join`

```rust
pub use rayon_core::join;
```

### Re-export `join_context`

```rust
pub use rayon_core::join_context;
```

### Re-export `spawn`

```rust
pub use rayon_core::spawn;
```

### Re-export `spawn_fifo`

```rust
pub use rayon_core::spawn_fifo;
```

### Re-export `yield_local`

```rust
pub use rayon_core::yield_local;
```

### Re-export `yield_now`

```rust
pub use rayon_core::yield_now;
```

### Re-export `Yield`

```rust
pub use rayon_core::Yield;
```

