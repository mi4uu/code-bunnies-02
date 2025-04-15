# Crate Documentation

**Version:** 0.7.5

**Format Version:** 43

# Module `litemap`

# `litemap`

`litemap` is a crate providing [`LiteMap`], a highly simplistic "flat" key-value map
based off of a single sorted vector.

The goal of this crate is to provide a map that is good enough for small
sizes, and does not carry the binary size impact of [`HashMap`](std::collections::HashMap)
or [`BTreeMap`](alloc::collections::BTreeMap).

If binary size is not a concern, [`std::collections::BTreeMap`] may be a better choice
for your use case. It behaves very similarly to [`LiteMap`] for less than 12 elements,
and upgrades itself gracefully for larger inputs.

## Pluggable Backends

By default, [`LiteMap`] is backed by a [`Vec`]; however, it can be backed by any appropriate
random-access data store, giving that data store a map-like interface. See the [`store`]
module for more details.

## Const construction

[`LiteMap`] supports const construction from any store that is const-constructible, such as a
static slice, via [`LiteMap::from_sorted_store_unchecked()`]. This also makes [`LiteMap`]
suitable for use with [`databake`]. See [`impl Bake for LiteMap`] for more details.

[`impl Bake for LiteMap`]: ./struct.LiteMap.html#impl-Bake-for-LiteMap<K,+V,+S>
[`Vec`]: alloc::vec::Vec

## Modules

## Module `store`

Traits for pluggable LiteMap backends.

By default, LiteMap is backed by a `Vec`. However, in some environments, it may be desirable
to use a different data store while still using LiteMap to manage proper ordering of items.

The general guidelines for a performant data store are:

1. Must support efficient random access for binary search
2. Should support efficient append operations for deserialization

To plug a custom data store into LiteMap, implement:

- [`Store`] for most of the methods
- [`StoreIterable`] for methods that return iterators
- [`StoreFromIterator`] to enable `FromIterator` for LiteMap

To test your implementation, enable the `"testing"` Cargo feature and use [`check_store()`].

[`check_store()`]: crate::testing::check_store

```rust
pub mod store { /* ... */ }
```

### Traits

#### Trait `StoreConstEmpty`

Trait to enable const construction of empty store.

```rust
pub trait StoreConstEmpty<K: ?Sized, V: ?Sized> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `EMPTY`: An empty store

##### Implementations

This trait is implemented for the following types:

- `&''a [(K, V)]` with <''a, K: ''a, V: ''a>
- `alloc::vec::Vec<(K, V)>` with <K, V>

#### Trait `Store`

Trait to enable pluggable backends for LiteMap.

Some methods have default implementations provided for convenience; however, it is generally
better to implement all methods that your data store supports.

```rust
pub trait Store<K: ?Sized, V: ?Sized>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `lm_len`: Returns the number of elements in the store.
- `lm_get`: Gets a key/value pair at the specified index.
- `lm_binary_search_by`: Searches the store for a particular element with a comparator function.

##### Provided Methods

- ```rust
  fn lm_is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the store is empty (contains 0 elements).

- ```rust
  fn lm_last(self: &Self) -> Option<(&K, &V)> { /* ... */ }
  ```
  Gets the last element in the store, or `None` if the store is empty.

##### Implementations

This trait is implemented for the following types:

- `&''a [(K, V)]` with <''a, K: ''a, V: ''a>
- `alloc::vec::Vec<(K, V)>` with <K, V>

#### Trait `StoreFromIterable`

```rust
pub trait StoreFromIterable<K, V>: Store<K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `lm_sort_from_iter`: Create a sorted store from `iter`.

##### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<(K, V)>` with <K: Ord, V>

#### Trait `StoreSlice`

```rust
pub trait StoreSlice<K: ?Sized, V: ?Sized>: Store<K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Slice`

###### Required Methods

- `lm_get_range`

##### Implementations

This trait is implemented for the following types:

- `&[(K, V)]` with <K, V>
- `alloc::vec::Vec<(K, V)>` with <K, V>

#### Trait `StoreMut`

```rust
pub trait StoreMut<K, V>: Store<K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `lm_with_capacity`: Creates a new store with the specified capacity hint.
- `lm_reserve`: Reserves additional capacity in the store.
- `lm_get_mut`: Gets a key/value pair at the specified index, with a mutable value.
- `lm_push`: Pushes one additional item onto the store.
- `lm_insert`: Inserts an item at a specific index in the store.
- `lm_remove`: Removes an item at a specific index in the store.
- `lm_clear`: Removes all items from the store.

##### Provided Methods

- ```rust
  fn lm_retain<F>(self: &mut Self, predicate: F)
where
    F: FnMut(&K, &V) -> bool { /* ... */ }
  ```
  Retains items satisfying a predicate in this store.

##### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<(K, V)>` with <K, V>

#### Trait `StoreIterable`

Iterator methods for the LiteMap store.

```rust
pub trait StoreIterable<''a, K: ''a + ?Sized, V: ''a + ?Sized>: Store<K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `KeyValueIter`

###### Required Methods

- `lm_iter`: Returns an iterator over key/value pairs.

##### Implementations

This trait is implemented for the following types:

- `&''a [(K, V)]` with <''a, K: ''a, V: ''a>
- `alloc::vec::Vec<(K, V)>` with <''a, K: ''a, V: ''a>

#### Trait `StoreIterableMut`

```rust
pub trait StoreIterableMut<''a, K: ''a, V: ''a>: StoreMut<K, V> + StoreIterable<''a, K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `KeyValueIterMut`

###### Required Methods

- `lm_iter_mut`: Returns an iterator over key/value pairs, with a mutable value.

##### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<(K, V)>` with <''a, K: ''a, V: ''a>

#### Trait `StoreIntoIterator`

```rust
pub trait StoreIntoIterator<K, V>: StoreMut<K, V> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `KeyValueIntoIter`

###### Required Methods

- `lm_into_iter`: Returns an iterator that moves every item from this store.

##### Provided Methods

- ```rust
  fn lm_extend_end(self: &mut Self, other: Self)
where
    Self: Sized { /* ... */ }
  ```
  Adds items from another store to the end of this store.

- ```rust
  fn lm_extend_start(self: &mut Self, other: Self)
where
    Self: Sized { /* ... */ }
  ```
  Adds items from another store to the beginning of this store.

##### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<(K, V)>` with <K, V>

#### Trait `StoreFromIterator`

A store that can be built from a tuple iterator.

```rust
pub trait StoreFromIterator<K, V>: FromIterator<(K, V)> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<(K, V)>` with <K, V>

## Re-exports

### Re-export `Entry`

```rust
pub use map::Entry;
```

### Re-export `LiteMap`

```rust
pub use map::LiteMap;
```

### Re-export `OccupiedEntry`

```rust
pub use map::OccupiedEntry;
```

### Re-export `VacantEntry`

```rust
pub use map::VacantEntry;
```

