# Crate Documentation

**Version:** 0.4.9

**Format Version:** 43

# Module `slab`

Pre-allocated storage for a uniform data type.

`Slab` provides pre-allocated storage for a single data type. If many values
of a single type are being allocated, it can be more efficient to
pre-allocate the necessary storage. Since the size of the type is uniform,
memory fragmentation can be avoided. Storing, clearing, and lookup
operations become very cheap.

While `Slab` may look like other Rust collections, it is not intended to be
used as a general purpose collection. The primary difference between `Slab`
and `Vec` is that `Slab` returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a
value associated with a given key is removed from a slab, that key may be
returned from future calls to `insert`.

# Examples

Basic storing and retrieval.

```
# use slab::*;
let mut slab = Slab::new();

let hello = slab.insert("hello");
let world = slab.insert("world");

assert_eq!(slab[hello], "hello");
assert_eq!(slab[world], "world");

slab[world] = "earth";
assert_eq!(slab[world], "earth");
```

Sometimes it is useful to be able to associate the key with the value being
inserted in the slab. This can be done with the `vacant_entry` API as such:

```
# use slab::*;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

It is generally a good idea to specify the desired capacity of a slab at
creation time. Note that `Slab` will grow the internal capacity when
attempting to insert a new value once the existing capacity has been reached.
To avoid this, add a check.

```
# use slab::*;
let mut slab = Slab::with_capacity(1024);

// ... use the slab

if slab.len() == slab.capacity() {
    panic!("slab full");
}

slab.insert("the slab is not at capacity yet");
```

# Capacity and reallocation

The capacity of a slab is the amount of space allocated for any future
values that will be inserted in the slab. This is not to be confused with
the *length* of the slab, which specifies the number of actual values
currently being inserted. If a slab's length is equal to its capacity, the
next value inserted into the slab will require growing the slab by
reallocating.

For example, a slab with capacity 10 and length 0 would be an empty slab
with space for 10 more stored values. Storing 10 or fewer elements into the
slab will not change its capacity or cause reallocation to occur. However,
if the slab length is increased to 11 (due to another `insert`), it will
have to reallocate, which can be slow. For this reason, it is recommended to
use [`Slab::with_capacity`] whenever possible to specify how many values the
slab is expected to store.

# Implementation

`Slab` is backed by a `Vec` of slots. Each slot is either occupied or
vacant. `Slab` maintains a stack of vacant slots using a linked list. To
find a vacant slot, the stack is popped. When a slot is released, it is
pushed onto the stack.

If there are no more available slots in the stack, then `Vec::reserve(1)` is
called and a new slot is created.

[`Slab::with_capacity`]: struct.Slab.html#with_capacity

## Types

### Struct `Slab`

Pre-allocated storage for a uniform data type

See the [module documentation] for more details.

[module documentation]: index.html

```rust
pub struct Slab<T> {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Construct a new, empty `Slab`.

- ```rust
  pub fn with_capacity(capacity: usize) -> Slab<T> { /* ... */ }
  ```
  Construct a new, empty `Slab` with the specified capacity.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of values the slab can store without reallocating.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for at least `additional` more values to be stored

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve the minimum capacity required to store exactly `additional`

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrink the capacity of the slab as much as possible without invalidating keys.

- ```rust
  pub fn compact<F>(self: &mut Self, rekey: F)
where
    F: FnMut(&mut T, usize, usize) -> bool { /* ... */ }
  ```
  Reduce the capacity as much as possible, changing the key for elements when necessary.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clear the slab of all values.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of stored values.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if there are no values stored in the slab.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Return an iterator over the slab.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```
  Return an iterator that allows modifying each value.

- ```rust
  pub fn get(self: &Self, key: usize) -> Option<&T> { /* ... */ }
  ```
  Return a reference to the value associated with the given key.

- ```rust
  pub fn get_mut(self: &mut Self, key: usize) -> Option<&mut T> { /* ... */ }
  ```
  Return a mutable reference to the value associated with the given key.

- ```rust
  pub fn get2_mut(self: &mut Self, key1: usize, key2: usize) -> Option<(&mut T, &mut T)> { /* ... */ }
  ```
  Return two mutable references to the values associated with the two

- ```rust
  pub unsafe fn get_unchecked(self: &Self, key: usize) -> &T { /* ... */ }
  ```
  Return a reference to the value associated with the given key without

- ```rust
  pub unsafe fn get_unchecked_mut(self: &mut Self, key: usize) -> &mut T { /* ... */ }
  ```
  Return a mutable reference to the value associated with the given key

- ```rust
  pub unsafe fn get2_unchecked_mut(self: &mut Self, key1: usize, key2: usize) -> (&mut T, &mut T) { /* ... */ }
  ```
  Return two mutable references to the values associated with the two

- ```rust
  pub fn key_of(self: &Self, present_element: &T) -> usize { /* ... */ }
  ```
  Get the key for an element in the slab.

- ```rust
  pub fn insert(self: &mut Self, val: T) -> usize { /* ... */ }
  ```
  Insert a value in the slab, returning key assigned to the value.

- ```rust
  pub fn vacant_key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key of the next vacant entry.

- ```rust
  pub fn vacant_entry(self: &mut Self) -> VacantEntry<''_, T> { /* ... */ }
  ```
  Return a handle to a vacant entry allowing for further manipulation.

- ```rust
  pub fn try_remove(self: &mut Self, key: usize) -> Option<T> { /* ... */ }
  ```
  Tries to remove the value associated with the given key,

- ```rust
  pub fn remove(self: &mut Self, key: usize) -> T { /* ... */ }
  ```
  Remove and return the value associated with the given key.

- ```rust
  pub fn contains(self: &Self, key: usize) -> bool { /* ... */ }
  ```
  Return `true` if a value is associated with the given key.

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(usize, &mut T) -> bool { /* ... */ }
  ```
  Retain only the elements specified by the predicate.

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, T> { /* ... */ }
  ```
  Return a draining iterator that removes all elements from the slab and

##### Trait Implementations

- **Sync**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, T> { /* ... */ }
    ```

- **Freeze**
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

- **UnwindSafe**
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, key: usize) -> &T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iterable: I) -> Self
where
    I: IntoIterator<Item = (usize, T)> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, key: usize) -> &mut T { /* ... */ }
    ```

### Struct `VacantEntry`

A handle to a vacant entry in a `Slab`.

`VacantEntry` allows constructing values with the key that they will be
assigned to.

# Examples

```
# use slab::*;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

```rust
pub struct VacantEntry<''a, T> {
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
  pub fn insert(self: Self, val: T) -> &''a mut T { /* ... */ }
  ```
  Insert a value in the entry, returning a mutable reference to the value.

- ```rust
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Return the key associated with this entry.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
### Struct `IntoIter`

A consuming iterator over the values stored in a `Slab`

```rust
pub struct IntoIter<T> {
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
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **FusedIterator**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `Iter`

An iterator over the values stored in the `Slab`

```rust
pub struct Iter<''a, T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

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

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **FusedIterator**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `IterMut`

A mutable iterator over the values stored in the `Slab`

```rust
pub struct IterMut<''a, T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **FusedIterator**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

### Struct `Drain`

A draining iterator for `Slab`

```rust
pub struct Drain<''a, T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **FusedIterator**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
