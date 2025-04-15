# Crate Documentation

**Version:** 0.5.1

**Format Version:** 43

# Module `arraydeque`

A circular buffer with fixed capacity.
Requires Rust 1.59+

It can be stored directly on the stack if needed.

This queue has `O(1)` amortized inserts and removals from both ends of the
container. It also has `O(1)` indexing like a vector. The contained elements
are not required to be copyable

This crate is inspired by [**bluss/arrayvec**](https://github.com/bluss/arrayvec)

# Feature Flags
The **arraydeque** crate has the following cargo feature flags:

- `std`
  - Optional, enabled by default
  - Conversions between `ArrayDeque` and `Vec`
  - Use libstd

# Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
arraydeque = "0.5"
```

Next, add this to your crate root:

```
extern crate arraydeque;
```

Currently arraydeque by default links to the standard library, but if you would
instead like to use arraydeque in a `#![no_std]` situation or crate you can
request this via:

```toml
[dependencies]
arraydeque = { version = "0.4", default-features = false }
```

# Behaviors

`ArrayDeque` provides two different behaviors, `Saturating` and `Wrapping`,
determining whether to remove existing element automatically when pushing
to a full deque.

See the [behavior module documentation](behavior/index.html) for more.

## Modules

## Module `behavior`

Behavior semantics for `ArrayDeque`.

`ArrayDeque` provides two different behaviors, `Saturating` and `Wrapping`,
determining whether to remove existing element automatically when pushing
to a full deque.

The behavior is indicated by a marker type parameter of `ArrayDeque`,
which defaults to `Saturating`.

## Saturating

Pushing any element when `ArrayDeque` is full will directly return an `Err(CapacityError)`
containing the element attempting to push, leaving the `ArrayDeque` unchanged.

```
use arraydeque::{ArrayDeque, Saturating, CapacityError};

let mut tester: ArrayDeque<_, 2, Saturating> = ArrayDeque::new();

assert_eq!(tester.push_back(1), Ok(()));
assert_eq!(tester.push_back(2), Ok(()));
assert_eq!(tester.push_back(3), Err(CapacityError { element: 3 }));
```

## Wrapping

Pushing any element when `ArrayDeque` is full will pop an element at
the other side to spare room.

```
use arraydeque::{ArrayDeque, Wrapping};

let mut tester: ArrayDeque<_, 2, Wrapping> = ArrayDeque::new();

assert_eq!(tester.push_back(1), None);
assert_eq!(tester.push_back(2), None);
assert_eq!(tester.push_back(3), Some(1));
```

```rust
pub mod behavior { /* ... */ }
```

### Types

#### Struct `Saturating`

Behavior for `ArrayDeque` that specifies saturating write semantics.

```rust
pub struct Saturating;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Behavior**
- **RefUnwindSafe**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
#### Struct `Wrapping`

Behavior for `ArrayDeque` that specifies wrapping write semantics.

```rust
pub struct Wrapping;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **Sync**
- **Send**
- **Freeze**
- **RefUnwindSafe**
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

- **Behavior**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
### Traits

#### Trait `Behavior`

Marker trait for indicating behaviors of `ArrayDeque`.

```rust
pub trait Behavior {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `Saturating`
- `Wrapping`

## Types

### Struct `ArrayDeque`

A fixed capacity ring buffer.

It can be stored directly on the stack if needed.

The "default" usage of this type as a queue is to use `push_back` to add to
the queue, and `pop_front` to remove from the queue. Iterating over `ArrayDeque` goes front
to back.

```rust
pub struct ArrayDeque<T, const CAP: usize, B: Behavior = Saturating> {
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
  pub fn push_front(self: &mut Self, element: T) -> Result<(), CapacityError<T>> { /* ... */ }
  ```
  Add an element to the front of the deque.

- ```rust
  pub fn push_back(self: &mut Self, element: T) -> Result<(), CapacityError<T>> { /* ... */ }
  ```
  Add an element to the back of the deque.

- ```rust
  pub fn insert(self: &mut Self, index: usize, element: T) -> Result<(), CapacityError<T>> { /* ... */ }
  ```
  Inserts an element at `index` within the `ArrayDeque`. Whichever

- ```rust
  pub fn extend_front<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Extend deque from front with the contents of an iterator.

- ```rust
  pub fn extend_back<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Extend deque from back with the contents of an iterator.

- ```rust
  pub fn push_front(self: &mut Self, element: T) -> Option<T> { /* ... */ }
  ```
  Add an element to the front of the deque.

- ```rust
  pub fn push_back(self: &mut Self, element: T) -> Option<T> { /* ... */ }
  ```
  Appends an element to the back of a buffer

- ```rust
  pub fn extend_front<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Extend deque from front with the contents of an iterator.

- ```rust
  pub fn extend_back<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Extend deque from back with the contents of an iterator.

- ```rust
  pub const fn new() -> ArrayDeque<T, CAP, B> { /* ... */ }
  ```
  Creates an empty `ArrayDeque`.

- ```rust
  pub const fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Return the capacity of the `ArrayDeque`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the `ArrayDeque`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the buffer contains no elements

- ```rust
  pub fn as_uninit_slice(self: &Self) -> &[MaybeUninit<T>] { /* ... */ }
  ```
  Entire capacity of the underlying storage

- ```rust
  pub fn as_uninit_slice_mut(self: &mut Self) -> &mut [MaybeUninit<T>] { /* ... */ }
  ```
  Entire capacity of the underlying storage

- ```rust
  pub fn is_full(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the buffer is full.

- ```rust
  pub fn contains(self: &Self, x: &T) -> bool
where
    T: PartialEq<T> { /* ... */ }
  ```
  Returns `true` if the `ArrayDeque` contains an element equal to the

- ```rust
  pub fn front(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Provides a reference to the front element, or `None` if the sequence is

- ```rust
  pub fn front_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Provides a mutable reference to the front element, or `None` if the

- ```rust
  pub fn back(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Provides a reference to the back element, or `None` if the sequence is

- ```rust
  pub fn back_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Provides a mutable reference to the back element, or `None` if the

- ```rust
  pub fn get(self: &Self, index: usize) -> Option<&T> { /* ... */ }
  ```
  Retrieves an element in the `ArrayDeque` by index.

- ```rust
  pub fn get_mut(self: &mut Self, index: usize) -> Option<&mut T> { /* ... */ }
  ```
  Retrieves an element in the `ArrayDeque` mutably by index.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Returns a front-to-back iterator.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```
  Returns a front-to-back iterator that returns mutable references.

- ```rust
  pub fn linearize(self: &mut Self) { /* ... */ }
  ```
  Make the buffer contiguous

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the first element and returns it, or `None` if the sequence is

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the last element from a buffer and returns it, or `None` if

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the buffer, removing all values.

- ```rust
  pub fn drain<R>(self: &mut Self, range: R) -> Drain<''_, T, CAP, B>
where
    R: RangeArgument<usize> { /* ... */ }
  ```
  Create a draining iterator that removes the specified range in the

- ```rust
  pub fn swap(self: &mut Self, i: usize, j: usize) { /* ... */ }
  ```
  Swaps elements at indices `i` and `j`.

- ```rust
  pub fn swap_remove_back(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Removes an element from anywhere in the `ArrayDeque` and returns it, replacing it with the

- ```rust
  pub fn swap_remove_front(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Removes an element from anywhere in the `ArrayDeque` and returns it,

- ```rust
  pub fn remove(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Removes and returns the element at `index` from the `ArrayDeque`.

- ```rust
  pub fn split_off(self: &mut Self, at: usize) -> Self { /* ... */ }
  ```
  Splits the collection into two at the given index.

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&T) -> bool { /* ... */ }
  ```
  Retains only the elements specified by the predicate.

- ```rust
  pub fn as_slices(self: &Self) -> (&[T], &[T]) { /* ... */ }
  ```
  Returns a pair of slices which contain, in order, the contents of the

- ```rust
  pub fn as_mut_slices(self: &mut Self) -> (&mut [T], &mut [T]) { /* ... */ }
  ```
  Returns a pair of slices which contain, in order, the contents of the

##### Trait Implementations

- **Eq**
- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
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

  - ```rust
    fn from(buf: ArrayDeque<T, CAP, Wrapping>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(buf: ArrayDeque<T, CAP, Saturating>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; N]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(deque: ArrayDeque<T, CAP, B>) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

- **Sync**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: usize) -> &mut T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Iter`

**Attributes:**

- `#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]`

`ArrayDeque` iterator

```rust
pub struct Iter<''a, T: ''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ExactSizeIterator**
- **Unpin**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Sync**
### Struct `IterMut`

**Attributes:**

- `#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]`

`ArrayDeque` mutable iterator

```rust
pub struct IterMut<''a, T: ''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ExactSizeIterator**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a mut T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a mut T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `IntoIter`

**Attributes:**

- `#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]`

By-value `ArrayDeque` iterator

```rust
pub struct IntoIter<T, const CAP: usize, B: Behavior> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Drain`

Draining `ArrayDeque` iterator

```rust
pub struct Drain<''a, T, const CAP: usize, B> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
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

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
## Re-exports

### Re-export `Saturating`

```rust
pub use behavior::Saturating;
```

### Re-export `Wrapping`

```rust
pub use behavior::Wrapping;
```

### Re-export `CapacityError`

```rust
pub use error::CapacityError;
```

### Re-export `RangeArgument`

```rust
pub use range::RangeArgument;
```

