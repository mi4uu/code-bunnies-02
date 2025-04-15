# Crate Documentation

**Version:** 0.5.2

**Format Version:** 43

# Module `dlv_list`

Crate that implements a semi-doubly linked list via a vector.

See [`VecList`] for more information.

# Features

By default, this crate uses the Rust standard library. To disable this, disable the default
`no_std` feature. Without this feature, certain methods will not be available.

## Types

### Struct `VecList`

A semi-doubly linked list implemented with a vector.

This provides many of the benefits of an actual linked list with a few tradeoffs. First, due to the use of an
underlying vector, an individual insert operation may be O(n) due to allocating more space for the vector. However,
it is amortized O(1) and it avoids the frequent allocations that traditional linked lists suffer from.

Another tradeoff is that extending a traditional linked list with another list is O(1) but a vector based
implementation is O(n). Splicing has a similar disadvantage.

Lastly, the vector based implementation is likely to have better cache locality in general.

```rust
pub struct VecList<T> {
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
  pub fn back(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns an immutable reference to the value at the back of the list, if it exists.

- ```rust
  pub fn back_index(self: &Self) -> Option<Index<T>> { /* ... */ }
  ```
  Returns the index of the value at the back of the list, if it exists.

- ```rust
  pub fn back_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Returns a mutable reference to the value at the back of the list, if it exists.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the capacity of the list.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Removes all values from the list and invalidates all existing indices.

- ```rust
  pub fn contains(self: &Self, value: &T) -> bool
where
    T: PartialEq { /* ... */ }
  ```
  Returns whether or not the list contains the given value.

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, T> { /* ... */ }
  ```
  Creates a draining iterator that removes all values from the list and yields them in order.

- ```rust
  pub fn front(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns an immutable reference to the value at the front of the list, if it exists.

- ```rust
  pub fn front_index(self: &Self) -> Option<Index<T>> { /* ... */ }
  ```
  Returns the index of the value at the front of the list, if it exists.

- ```rust
  pub fn front_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Returns a mutable reference to the value at the front of the list, if it exists.

- ```rust
  pub fn get(self: &Self, index: Index<T>) -> Option<&T> { /* ... */ }
  ```
  Returns an immutable reference to the value at the given index.

- ```rust
  pub unsafe fn get_unchecked(self: &Self, index: Index<T>) -> &T { /* ... */ }
  ```
  Returns an immutable reference to the value at the given index.

- ```rust
  pub fn get_mut(self: &mut Self, index: Index<T>) -> Option<&mut T> { /* ... */ }
  ```
  Returns a mutable reference to the value at the given index.

- ```rust
  pub unsafe fn get_unchecked_mut(self: &mut Self, index: Index<T>) -> &mut T { /* ... */ }
  ```
  Returns an mutable reference to the value at the given index.

- ```rust
  pub fn get_next_index(self: &Self, index: Index<T>) -> Option<Index<T>> { /* ... */ }
  ```
  Returns the index of the value next to the value at the given index.

- ```rust
  pub fn get_previous_index(self: &Self, index: Index<T>) -> Option<Index<T>> { /* ... */ }
  ```
  Returns the index of the value previous to the value at the given index.

- ```rust
  pub fn move_after(self: &mut Self, index: Index<T>, target: Index<T>) { /* ... */ }
  ```
  Move the node at `index` to after the node at `target`.

- ```rust
  pub fn move_before(self: &mut Self, index: Index<T>, target: Index<T>) { /* ... */ }
  ```
  Move the node at `index` to before the node at `target`.

- ```rust
  pub fn indices(self: &Self) -> Indices<''_, T> { /* ... */ }
  ```
  Creates an indices iterator which will yield all indices of the list in order.

- ```rust
  pub fn insert_after(self: &mut Self, index: Index<T>, value: T) -> Index<T> { /* ... */ }
  ```
  Inserts the given value after the value at the given index.

- ```rust
  pub fn insert_before(self: &mut Self, index: Index<T>, value: T) -> Index<T> { /* ... */ }
  ```
  Inserts the given value before the value at the given index.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether or not the list is empty.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to values in the list in order.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```
  Creates an iterator that yields mutable references to values in the list in order.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of values in the list.

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new list with no initial capacity.

- ```rust
  pub fn pack_to(self: &mut Self, minimum_capacity: usize) -> HashMap<Index<T>, Index<T>> { /* ... */ }
  ```
  Reorganizes the existing values to ensure maximum cache locality and shrinks the list such that the capacity is

- ```rust
  pub fn pack_to_fit(self: &mut Self) -> HashMap<Index<T>, Index<T>> { /* ... */ }
  ```
  Reorganizes the existing values to ensure maximum cache locality and shrinks the list such that no additional

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes and returns the value at the back of the list, if it exists.

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes and returns the value at the front of the list, if it exists.

- ```rust
  pub fn push_back(self: &mut Self, value: T) -> Index<T> { /* ... */ }
  ```
  Inserts the given value to the back of the list.

- ```rust
  pub fn push_front(self: &mut Self, value: T) -> Index<T> { /* ... */ }
  ```
  Inserts the given value to the front of the list.

- ```rust
  pub fn remove(self: &mut Self, index: Index<T>) -> Option<T> { /* ... */ }
  ```
  Removes and returns the value at the given index, if it exists.

- ```rust
  pub fn reserve(self: &mut Self, additional_capacity: usize) { /* ... */ }
  ```
  Reserves capacity for the given expected size increase.

- ```rust
  pub fn retain<Predicate>(self: &mut Self, predicate: Predicate)
where
    Predicate: FnMut(&mut T) -> bool { /* ... */ }
  ```
  Removes all elements from the list not satisfying the given predicate.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Creates a new list with the given capacity.

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &LinkedList<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &VecList<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Vec<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &VecList<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[T; N]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &VecList<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a [T]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &VecList<T>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: Index<T>) -> &mut <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<StateHasher>(self: &Self, state: &mut StateHasher)
where
    StateHasher: Hasher { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<Iter>(self: &mut Self, iter: Iter)
where
    Iter: IntoIterator<Item = T> { /* ... */ }
    ```

  - ```rust
    fn extend<Iter>(self: &mut Self, iter: Iter)
where
    Iter: IntoIterator<Item = &''a T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: Index<T>) -> &<Self as >::Output { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **FromIterator**
  - ```rust
    fn from_iter<Iter>(iter: Iter) -> Self
where
    Iter: IntoIterator<Item = T> { /* ... */ }
    ```

- **Freeze**
### Struct `Index`

A wrapper type that indicates an index into the list.

This index may be invalidated by operations on the list itself.

```rust
pub struct Index<T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: Index<T>) -> &mut <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Index**
  - ```rust
    fn index(self: &Self, index: Index<T>) -> &<Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **Send**
- **Hash**
  - ```rust
    fn hash<StateHasher>(self: &Self, hasher: &mut StateHasher)
where
    StateHasher: Hasher { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Drain`

An iterator that yields and removes all entries from the list.

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

##### Methods

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to entries in the list.

##### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **FusedIterator**
- **RefUnwindSafe**
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

- **Unpin**
- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
### Struct `Indices`

An iterator that yields all indices in the list.

```rust
pub struct Indices<''a, T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `IntoIter`

An iterator that moves all entries out of the entry list.

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

##### Methods

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to entries in the list.

##### Trait Implementations

- **Sync**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter<T> { /* ... */ }
    ```

- **UnwindSafe**
- **ExactSizeIterator**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FusedIterator**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

### Struct `Iter`

An iterator that yields immutable references to entries in the list.

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

- **ExactSizeIterator**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<''a, T> { /* ... */ }
    ```

- **FusedIterator**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

### Struct `IterMut`

An iterator that yields mutable references to entries in the list.

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

##### Methods

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to entries in the list.

##### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **ExactSizeIterator**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FusedIterator**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
