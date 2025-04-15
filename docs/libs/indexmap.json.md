# Crate Documentation

**Version:** 2.9.0

**Format Version:** 43

# Module `indexmap`

[`IndexMap`] is a hash table where the iteration order of the key-value
pairs is independent of the hash values of the keys.

[`IndexSet`] is a corresponding hash set using the same implementation and
with similar properties.

### Highlights

[`IndexMap`] and [`IndexSet`] are drop-in compatible with the std `HashMap`
and `HashSet`, but they also have some features of note:

- The ordering semantics (see their documentation for details)
- Sorting methods and the [`.pop()`][IndexMap::pop] methods.
- The [`Equivalent`] trait, which offers more flexible equality definitions
  between borrowed and owned versions of keys.
- The [`MutableKeys`][map::MutableKeys] trait, which gives opt-in mutable
  access to map keys, and [`MutableValues`][set::MutableValues] for sets.

### Feature Flags

To reduce the amount of compiled code in the crate by default, certain
features are gated behind [feature flags]. These allow you to opt in to (or
out of) functionality. Below is a list of the features available in this
crate.

* `std`: Enables features which require the Rust standard library. For more
  information see the section on [`no_std`].
* `rayon`: Enables parallel iteration and other parallel methods.
* `serde`: Adds implementations for [`Serialize`] and [`Deserialize`]
  to [`IndexMap`] and [`IndexSet`]. Alternative implementations for
  (de)serializing [`IndexMap`] as an ordered sequence are available in the
  [`map::serde_seq`] module.
* `arbitrary`: Adds implementations for the [`arbitrary::Arbitrary`] trait
  to [`IndexMap`] and [`IndexSet`].
* `quickcheck`: Adds implementations for the [`quickcheck::Arbitrary`] trait
  to [`IndexMap`] and [`IndexSet`].
* `borsh` (**deprecated**): Adds implementations for [`BorshSerialize`] and
  [`BorshDeserialize`] to [`IndexMap`] and [`IndexSet`]. Due to a cyclic
  dependency that arose between [`borsh`] and `indexmap`, `borsh v1.5.6`
  added an `indexmap` feature that should be used instead of enabling the
  feature here.

_Note: only the `std` feature is enabled by default._

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[`no_std`]: #no-standard-library-targets
[`Serialize`]: `::serde::Serialize`
[`Deserialize`]: `::serde::Deserialize`
[`BorshSerialize`]: `::borsh::BorshSerialize`
[`BorshDeserialize`]: `::borsh::BorshDeserialize`
[`borsh`]: `::borsh`
[`arbitrary::Arbitrary`]: `::arbitrary::Arbitrary`
[`quickcheck::Arbitrary`]: `::quickcheck::Arbitrary`

### Alternate Hashers

[`IndexMap`] and [`IndexSet`] have a default hasher type
[`S = RandomState`][std::collections::hash_map::RandomState],
just like the standard `HashMap` and `HashSet`, which is resistant to
HashDoS attacks but not the most performant. Type aliases can make it easier
to use alternate hashers:

```
use fnv::FnvBuildHasher;
use indexmap::{IndexMap, IndexSet};

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;
type FnvIndexSet<T> = IndexSet<T, FnvBuildHasher>;

let std: IndexSet<i32> = (0..100).collect();
let fnv: FnvIndexSet<i32> = (0..100).collect();
assert_eq!(std, fnv);
```

### Rust Version

This version of indexmap requires Rust 1.63 or later.

The indexmap 2.x release series will use a carefully considered version
upgrade policy, where in a later 2.x version, we will raise the minimum
required Rust version.

## No Standard Library Targets

This crate supports being built without `std`, requiring `alloc` instead.
This is chosen by disabling the default "std" cargo feature, by adding
`default-features = false` to your dependency specification.

- Creating maps and sets using [`new`][IndexMap::new] and
  [`with_capacity`][IndexMap::with_capacity] is unavailable without `std`.
  Use methods [`IndexMap::default`], [`with_hasher`][IndexMap::with_hasher],
  [`with_capacity_and_hasher`][IndexMap::with_capacity_and_hasher] instead.
  A no-std compatible hasher will be needed as well, for example
  from the crate `twox-hash`.
- Macros [`indexmap!`] and [`indexset!`] are unavailable without `std`. Use
  the macros [`indexmap_with_default!`] and [`indexset_with_default!`] instead.

## Modules

## Module `map`

[`IndexMap`] is a hash table where the iteration order of the key-value
pairs is independent of the hash values of the keys.

```rust
pub mod map { /* ... */ }
```

### Types

#### Struct `IndexMap`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A hash table where the iteration order of the key-value pairs is independent
of the hash values of the keys.

The interface is closely compatible with the standard
[`HashMap`][std::collections::HashMap],
but also has additional features.

# Order

The key-value pairs have a consistent order that is determined by
the sequence of insertion and removal calls on the map. The order does
not depend on the keys or the hash function at all.

All iterators traverse the map in *the order*.

The insertion order is preserved, with **notable exceptions** like the
[`.remove()`][Self::remove] or [`.swap_remove()`][Self::swap_remove] methods.
Methods such as [`.sort_by()`][Self::sort_by] of
course result in a new order, depending on the sorting order.

# Indices

The key-value pairs are indexed in a compact range without holes in the
range `0..self.len()`. For example, the method `.get_full` looks up the
index for a key, and the method `.get_index` looks up the key-value pair by
index.

# Examples

```
use indexmap::IndexMap;

// count the frequency of each letter in a sentence.
let mut letters = IndexMap::new();
for ch in "a short treatise on fungi".chars() {
    *letters.entry(ch).or_insert(0) += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

```rust
pub struct IndexMap<K, V, S = std::collections::hash_map::RandomState> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new map. (Does not allocate.)

- ```rust
  pub fn with_capacity(n: usize) -> Self { /* ... */ }
  ```
  Create a new map with capacity for `n` key-value pairs. (Does not

- ```rust
  pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self { /* ... */ }
  ```
  Create a new map with capacity for `n` key-value pairs. (Does not

- ```rust
  pub const fn with_hasher(hash_builder: S) -> Self { /* ... */ }
  ```
  Create a new map with `hash_builder`.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of elements the map can hold without reallocating.

- ```rust
  pub fn hasher(self: &Self) -> &S { /* ... */ }
  ```
  Return a reference to the map's `BuildHasher`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of key-value pairs in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the map contains no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V> { /* ... */ }
  ```
  Return an iterator over the key-value pairs of the map, in their order

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, K, V> { /* ... */ }
  ```
  Return an iterator over the key-value pairs of the map, in their order

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, K, V> { /* ... */ }
  ```
  Return an iterator over the keys of the map, in their order

- ```rust
  pub fn into_keys(self: Self) -> IntoKeys<K, V> { /* ... */ }
  ```
  Return an owning iterator over the keys of the map, in their order

- ```rust
  pub fn values(self: &Self) -> Values<''_, K, V> { /* ... */ }
  ```
  Return an iterator over the values of the map, in their order

- ```rust
  pub fn values_mut(self: &mut Self) -> ValuesMut<''_, K, V> { /* ... */ }
  ```
  Return an iterator over mutable references to the values of the map,

- ```rust
  pub fn into_values(self: Self) -> IntoValues<K, V> { /* ... */ }
  ```
  Return an owning iterator over the values of the map, in their order

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Remove all key-value pairs in the map, while preserving its capacity.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shortens the map, keeping the first `len` elements and dropping the rest.

- ```rust
  pub fn drain<R>(self: &mut Self, range: R) -> Drain<''_, K, V>
where
    R: RangeBounds<usize> { /* ... */ }
  ```
  Clears the `IndexMap` in the given index range, returning those

- ```rust
  pub fn split_off(self: &mut Self, at: usize) -> Self
where
    S: Clone { /* ... */ }
  ```
  Splits the collection into two at the given index.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more key-value pairs.

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more key-value pairs, without over-allocating.

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```
  Try to reserve capacity for `additional` more key-value pairs.

- ```rust
  pub fn try_reserve_exact(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```
  Try to reserve capacity for `additional` more key-value pairs, without over-allocating.

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrink the capacity of the map as much as possible.

- ```rust
  pub fn shrink_to(self: &mut Self, min_capacity: usize) { /* ... */ }
  ```
  Shrink the capacity of the map with a lower limit.

- ```rust
  pub fn insert(self: &mut Self, key: K, value: V) -> Option<V> { /* ... */ }
  ```
  Insert a key-value pair in the map.

- ```rust
  pub fn insert_full(self: &mut Self, key: K, value: V) -> (usize, Option<V>) { /* ... */ }
  ```
  Insert a key-value pair in the map, and get their index.

- ```rust
  pub fn insert_sorted(self: &mut Self, key: K, value: V) -> (usize, Option<V>)
where
    K: Ord { /* ... */ }
  ```
  Insert a key-value pair in the map at its ordered position among sorted keys.

- ```rust
  pub fn insert_before(self: &mut Self, index: usize, key: K, value: V) -> (usize, Option<V>) { /* ... */ }
  ```
  Insert a key-value pair in the map before the entry at the given index, or at the end.

- ```rust
  pub fn shift_insert(self: &mut Self, index: usize, key: K, value: V) -> Option<V> { /* ... */ }
  ```
  Insert a key-value pair in the map at the given index.

- ```rust
  pub fn entry(self: &mut Self, key: K) -> Entry<''_, K, V> { /* ... */ }
  ```
  Get the given key’s corresponding entry in the map for insertion and/or

- ```rust
  pub fn splice<R, I>(self: &mut Self, range: R, replace_with: I) -> Splice<''_, <I as >::IntoIter, K, V, S>
where
    R: RangeBounds<usize>,
    I: IntoIterator<Item = (K, V)> { /* ... */ }
  ```
  Creates a splicing iterator that replaces the specified range in the map

- ```rust
  pub fn append<S2>(self: &mut Self, other: &mut IndexMap<K, V, S2>) { /* ... */ }
  ```
  Moves all key-value pairs from `other` into `self`, leaving `other` empty.

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return `true` if an equivalent to `key` exists in the map.

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&V>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return a reference to the value stored for `key`, if it is present,

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return references to the key-value pair stored for `key`,

- ```rust
  pub fn get_full<Q>(self: &Self, key: &Q) -> Option<(usize, &K, &V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return item index, key and value

- ```rust
  pub fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return item index, if it exists in the map

- ```rust
  pub fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut V>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```

- ```rust
  pub fn get_full_mut<Q>(self: &mut Self, key: &Q) -> Option<(usize, &K, &mut V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```

- ```rust
  pub fn get_disjoint_mut<Q, const N: usize>(self: &mut Self, keys: [&Q; N]) -> [Option<&mut V>; N]
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Return the values for `N` keys. If any key is duplicated, this function will panic.

- ```rust
  pub fn remove<Q>(self: &mut Self, key: &Q) -> Option<V>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove the key-value pair equivalent to `key` and return

- ```rust
  pub fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(K, V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove and return the key-value pair equivalent to `key`.

- ```rust
  pub fn swap_remove<Q>(self: &mut Self, key: &Q) -> Option<V>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove the key-value pair equivalent to `key` and return

- ```rust
  pub fn swap_remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(K, V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove and return the key-value pair equivalent to `key`.

- ```rust
  pub fn swap_remove_full<Q>(self: &mut Self, key: &Q) -> Option<(usize, K, V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove the key-value pair equivalent to `key` and return it and

- ```rust
  pub fn shift_remove<Q>(self: &mut Self, key: &Q) -> Option<V>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove the key-value pair equivalent to `key` and return

- ```rust
  pub fn shift_remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(K, V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove and return the key-value pair equivalent to `key`.

- ```rust
  pub fn shift_remove_full<Q>(self: &mut Self, key: &Q) -> Option<(usize, K, V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
  ```
  Remove the key-value pair equivalent to `key` and return it and

- ```rust
  pub fn pop(self: &mut Self) -> Option<(K, V)> { /* ... */ }
  ```
  Remove the last key-value pair

- ```rust
  pub fn retain<F>(self: &mut Self, keep: F)
where
    F: FnMut(&K, &mut V) -> bool { /* ... */ }
  ```
  Scan through each key-value pair in the map and keep those where the

- ```rust
  pub fn sort_keys(self: &mut Self)
where
    K: Ord { /* ... */ }
  ```
  Sort the map’s key-value pairs by the default ordering of the keys.

- ```rust
  pub fn sort_by<F>(self: &mut Self, cmp: F)
where
    F: FnMut(&K, &V, &K, &V) -> Ordering { /* ... */ }
  ```
  Sort the map’s key-value pairs in place using the comparison

- ```rust
  pub fn sorted_by<F>(self: Self, cmp: F) -> IntoIter<K, V>
where
    F: FnMut(&K, &V, &K, &V) -> Ordering { /* ... */ }
  ```
  Sort the key-value pairs of the map and return a by-value iterator of

- ```rust
  pub fn sort_unstable_keys(self: &mut Self)
where
    K: Ord { /* ... */ }
  ```
  Sort the map's key-value pairs by the default ordering of the keys, but

- ```rust
  pub fn sort_unstable_by<F>(self: &mut Self, cmp: F)
where
    F: FnMut(&K, &V, &K, &V) -> Ordering { /* ... */ }
  ```
  Sort the map's key-value pairs in place using the comparison function `cmp`, but

- ```rust
  pub fn sorted_unstable_by<F>(self: Self, cmp: F) -> IntoIter<K, V>
where
    F: FnMut(&K, &V, &K, &V) -> Ordering { /* ... */ }
  ```
  Sort the key-value pairs of the map and return a by-value iterator of

- ```rust
  pub fn sort_by_cached_key<T, F>(self: &mut Self, sort_key: F)
where
    T: Ord,
    F: FnMut(&K, &V) -> T { /* ... */ }
  ```
  Sort the map’s key-value pairs in place using a sort-key extraction function.

- ```rust
  pub fn binary_search_keys(self: &Self, x: &K) -> Result<usize, usize>
where
    K: Ord { /* ... */ }
  ```
  Search over a sorted map for a key.

- ```rust
  pub fn binary_search_by<''a, F>(self: &''a Self, f: F) -> Result<usize, usize>
where
    F: FnMut(&''a K, &''a V) -> Ordering { /* ... */ }
  ```
  Search over a sorted map with a comparator function.

- ```rust
  pub fn binary_search_by_key<''a, B, F>(self: &''a Self, b: &B, f: F) -> Result<usize, usize>
where
    F: FnMut(&''a K, &''a V) -> B,
    B: Ord { /* ... */ }
  ```
  Search over a sorted map with an extraction function.

- ```rust
  pub fn partition_point<P>(self: &Self, pred: P) -> usize
where
    P: FnMut(&K, &V) -> bool { /* ... */ }
  ```
  Returns the index of the partition point of a sorted map according to the given predicate

- ```rust
  pub fn reverse(self: &mut Self) { /* ... */ }
  ```
  Reverses the order of the map’s key-value pairs in place.

- ```rust
  pub fn as_slice(self: &Self) -> &Slice<K, V> { /* ... */ }
  ```
  Returns a slice of all the key-value pairs in the map.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut Slice<K, V> { /* ... */ }
  ```
  Returns a mutable slice of all the key-value pairs in the map.

- ```rust
  pub fn into_boxed_slice(self: Self) -> Box<Slice<K, V>> { /* ... */ }
  ```
  Converts into a boxed slice of all the key-value pairs in the map.

- ```rust
  pub fn get_index(self: &Self, index: usize) -> Option<(&K, &V)> { /* ... */ }
  ```
  Get a key-value pair by index

- ```rust
  pub fn get_index_mut(self: &mut Self, index: usize) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Get a key-value pair by index

- ```rust
  pub fn get_index_entry(self: &mut Self, index: usize) -> Option<IndexedEntry<''_, K, V>> { /* ... */ }
  ```
  Get an entry in the map by index for in-place manipulation.

- ```rust
  pub fn get_disjoint_indices_mut<const N: usize>(self: &mut Self, indices: [usize; N]) -> Result<[(&K, &mut V); N], GetDisjointMutError> { /* ... */ }
  ```
  Get an array of `N` key-value pairs by `N` indices

- ```rust
  pub fn get_range<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<&Slice<K, V>> { /* ... */ }
  ```
  Returns a slice of key-value pairs in the given range of indices.

- ```rust
  pub fn get_range_mut<R: RangeBounds<usize>>(self: &mut Self, range: R) -> Option<&mut Slice<K, V>> { /* ... */ }
  ```
  Returns a mutable slice of key-value pairs in the given range of indices.

- ```rust
  pub fn first(self: &Self) -> Option<(&K, &V)> { /* ... */ }
  ```
  Get the first key-value pair

- ```rust
  pub fn first_mut(self: &mut Self) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Get the first key-value pair, with mutable access to the value

- ```rust
  pub fn first_entry(self: &mut Self) -> Option<IndexedEntry<''_, K, V>> { /* ... */ }
  ```
  Get the first entry in the map for in-place manipulation.

- ```rust
  pub fn last(self: &Self) -> Option<(&K, &V)> { /* ... */ }
  ```
  Get the last key-value pair

- ```rust
  pub fn last_mut(self: &mut Self) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Get the last key-value pair, with mutable access to the value

- ```rust
  pub fn last_entry(self: &mut Self) -> Option<IndexedEntry<''_, K, V>> { /* ... */ }
  ```
  Get the last entry in the map for in-place manipulation.

- ```rust
  pub fn swap_remove_index(self: &mut Self, index: usize) -> Option<(K, V)> { /* ... */ }
  ```
  Remove the key-value pair by index

- ```rust
  pub fn shift_remove_index(self: &mut Self, index: usize) -> Option<(K, V)> { /* ... */ }
  ```
  Remove the key-value pair by index

- ```rust
  pub fn move_index(self: &mut Self, from: usize, to: usize) { /* ... */ }
  ```
  Moves the position of a key-value pair from one index to another

- ```rust
  pub fn swap_indices(self: &mut Self, a: usize, b: usize) { /* ... */ }
  ```
  Swaps the position of two key-value pairs in the map.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Return an empty [`IndexMap`]

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, range: ops::Range<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeFrom<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeFull) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeTo<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeToInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: (Bound<usize>, Bound<usize>)) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, key: &Q) -> &V { /* ... */ }
    ```
    Returns a reference to the value corresponding to the supplied `key`.

  - ```rust
    fn index(self: &Self, index: usize) -> &V { /* ... */ }
    ```
    Returns a reference to the value at the supplied `index`.

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, other: &Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self { /* ... */ }
    ```
    Create an `IndexMap` from the sequence of key-value pairs in the

- **MutableKeys**
  - ```rust
    fn get_full_mut2<Q>(self: &mut Self, key: &Q) -> Option<(usize, &mut K, &mut V)>
where
    Q: ?Sized + Hash + Equivalent<K> { /* ... */ }
    ```

  - ```rust
    fn get_index_mut2(self: &mut Self, index: usize) -> Option<(&mut K, &mut V)> { /* ... */ }
    ```

  - ```rust
    fn iter_mut2(self: &mut Self) -> IterMut2<''_, <Self as >::Key, <Self as >::Value> { /* ... */ }
    ```

  - ```rust
    fn retain2<F>(self: &mut Self, keep: F)
where
    F: FnMut(&mut K, &mut V) -> bool { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, range: ops::Range<usize>) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: ops::RangeFrom<usize>) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: ops::RangeFull) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: ops::RangeInclusive<usize>) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: ops::RangeTo<usize>) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: ops::RangeToInclusive<usize>) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, range: (Bound<usize>, Bound<usize>)) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, key: &Q) -> &mut V { /* ... */ }
    ```
    Returns a mutable reference to the value corresponding to the supplied `key`.

  - ```rust
    fn index_mut(self: &mut Self, index: usize) -> &mut V { /* ... */ }
    ```
    Returns a mutable reference to the value at the supplied `index`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(arr: [(K, V); N]) -> Self { /* ... */ }
    ```
    # Examples

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = (K, V)>>(self: &mut Self, iterable: I) { /* ... */ }
    ```
    Extend the map with all key-value pairs in the iterable.

  - ```rust
    fn extend<I: IntoIterator<Item = (&''a K, &''a V)>>(self: &mut Self, iterable: I) { /* ... */ }
    ```
    Extend the map with all key-value pairs in the iterable.

- **RawEntryApiV1**
  - ```rust
    fn raw_entry_v1(self: &Self) -> RawEntryBuilder<''_, K, V, S> { /* ... */ }
    ```

  - ```rust
    fn raw_entry_mut_v1(self: &mut Self) -> RawEntryBuilderMut<''_, K, V, S> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IndexMap<K, V2, S2>) -> bool { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `raw_entry_v1`

```rust
pub use self::core::raw_entry_v1;
```

#### Re-export `RawEntryApiV1`

```rust
pub use self::core::raw_entry_v1::RawEntryApiV1;
```

#### Re-export `Entry`

```rust
pub use self::core::Entry;
```

#### Re-export `IndexedEntry`

```rust
pub use self::core::IndexedEntry;
```

#### Re-export `OccupiedEntry`

```rust
pub use self::core::OccupiedEntry;
```

#### Re-export `VacantEntry`

```rust
pub use self::core::VacantEntry;
```

#### Re-export `Drain`

```rust
pub use self::iter::Drain;
```

#### Re-export `IntoIter`

```rust
pub use self::iter::IntoIter;
```

#### Re-export `IntoKeys`

```rust
pub use self::iter::IntoKeys;
```

#### Re-export `IntoValues`

```rust
pub use self::iter::IntoValues;
```

#### Re-export `Iter`

```rust
pub use self::iter::Iter;
```

#### Re-export `IterMut`

```rust
pub use self::iter::IterMut;
```

#### Re-export `IterMut2`

```rust
pub use self::iter::IterMut2;
```

#### Re-export `Keys`

```rust
pub use self::iter::Keys;
```

#### Re-export `Splice`

```rust
pub use self::iter::Splice;
```

#### Re-export `Values`

```rust
pub use self::iter::Values;
```

#### Re-export `ValuesMut`

```rust
pub use self::iter::ValuesMut;
```

#### Re-export `MutableEntryKey`

```rust
pub use self::mutable::MutableEntryKey;
```

#### Re-export `MutableKeys`

```rust
pub use self::mutable::MutableKeys;
```

#### Re-export `Slice`

```rust
pub use self::slice::Slice;
```

## Module `set`

A hash set implemented using [`IndexMap`]

```rust
pub mod set { /* ... */ }
```

### Types

#### Struct `IndexSet`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A hash set where the iteration order of the values is independent of their
hash values.

The interface is closely compatible with the standard
[`HashSet`][std::collections::HashSet],
but also has additional features.

# Order

The values have a consistent order that is determined by the sequence of
insertion and removal calls on the set. The order does not depend on the
values or the hash function at all. Note that insertion order and value
are not affected if a re-insertion is attempted once an element is
already present.

All iterators traverse the set *in order*.  Set operation iterators like
[`IndexSet::union`] produce a concatenated order, as do their matching "bitwise"
operators.  See their documentation for specifics.

The insertion order is preserved, with **notable exceptions** like the
[`.remove()`][Self::remove] or [`.swap_remove()`][Self::swap_remove] methods.
Methods such as [`.sort_by()`][Self::sort_by] of
course result in a new order, depending on the sorting order.

# Indices

The values are indexed in a compact range without holes in the range
`0..self.len()`. For example, the method `.get_full` looks up the index for
a value, and the method `.get_index` looks up the value by index.

# Complexity

Internally, `IndexSet<T, S>` just holds an [`IndexMap<T, (), S>`](IndexMap). Thus the complexity
of the two are the same for most methods.

# Examples

```
use indexmap::IndexSet;

// Collects which letters appear in a sentence.
let letters: IndexSet<_> = "a short treatise on fungi".chars().collect();

assert!(letters.contains(&'s'));
assert!(letters.contains(&'t'));
assert!(letters.contains(&'u'));
assert!(!letters.contains(&'y'));
```

```rust
pub struct IndexSet<T, S = std::collections::hash_map::RandomState> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new set. (Does not allocate.)

- ```rust
  pub fn with_capacity(n: usize) -> Self { /* ... */ }
  ```
  Create a new set with capacity for `n` elements.

- ```rust
  pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self { /* ... */ }
  ```
  Create a new set with capacity for `n` elements.

- ```rust
  pub const fn with_hasher(hash_builder: S) -> Self { /* ... */ }
  ```
  Create a new set with `hash_builder`.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of elements the set can hold without reallocating.

- ```rust
  pub fn hasher(self: &Self) -> &S { /* ... */ }
  ```
  Return a reference to the set's `BuildHasher`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of elements in the set.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the set contains no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  Return an iterator over the values of the set, in their order

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Remove all elements in the set, while preserving its capacity.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```
  Shortens the set, keeping the first `len` elements and dropping the rest.

- ```rust
  pub fn drain<R>(self: &mut Self, range: R) -> Drain<''_, T>
where
    R: RangeBounds<usize> { /* ... */ }
  ```
  Clears the `IndexSet` in the given index range, returning those values

- ```rust
  pub fn split_off(self: &mut Self, at: usize) -> Self
where
    S: Clone { /* ... */ }
  ```
  Splits the collection into two at the given index.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more values.

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserve capacity for `additional` more values, without over-allocating.

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```
  Try to reserve capacity for `additional` more values.

- ```rust
  pub fn try_reserve_exact(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```
  Try to reserve capacity for `additional` more values, without over-allocating.

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrink the capacity of the set as much as possible.

- ```rust
  pub fn shrink_to(self: &mut Self, min_capacity: usize) { /* ... */ }
  ```
  Shrink the capacity of the set with a lower limit.

- ```rust
  pub fn insert(self: &mut Self, value: T) -> bool { /* ... */ }
  ```
  Insert the value into the set.

- ```rust
  pub fn insert_full(self: &mut Self, value: T) -> (usize, bool) { /* ... */ }
  ```
  Insert the value into the set, and get its index.

- ```rust
  pub fn insert_sorted(self: &mut Self, value: T) -> (usize, bool)
where
    T: Ord { /* ... */ }
  ```
  Insert the value into the set at its ordered position among sorted values.

- ```rust
  pub fn insert_before(self: &mut Self, index: usize, value: T) -> (usize, bool) { /* ... */ }
  ```
  Insert the value into the set before the value at the given index, or at the end.

- ```rust
  pub fn shift_insert(self: &mut Self, index: usize, value: T) -> bool { /* ... */ }
  ```
  Insert the value into the set at the given index.

- ```rust
  pub fn replace(self: &mut Self, value: T) -> Option<T> { /* ... */ }
  ```
  Adds a value to the set, replacing the existing value, if any, that is

- ```rust
  pub fn replace_full(self: &mut Self, value: T) -> (usize, Option<T>) { /* ... */ }
  ```
  Adds a value to the set, replacing the existing value, if any, that is

- ```rust
  pub fn difference<''a, S2>(self: &''a Self, other: &''a IndexSet<T, S2>) -> Difference<''a, T, S2>
where
    S2: BuildHasher { /* ... */ }
  ```
  Return an iterator over the values that are in `self` but not `other`.

- ```rust
  pub fn symmetric_difference<''a, S2>(self: &''a Self, other: &''a IndexSet<T, S2>) -> SymmetricDifference<''a, T, S, S2>
where
    S2: BuildHasher { /* ... */ }
  ```
  Return an iterator over the values that are in `self` or `other`,

- ```rust
  pub fn intersection<''a, S2>(self: &''a Self, other: &''a IndexSet<T, S2>) -> Intersection<''a, T, S2>
where
    S2: BuildHasher { /* ... */ }
  ```
  Return an iterator over the values that are in both `self` and `other`.

- ```rust
  pub fn union<''a, S2>(self: &''a Self, other: &''a IndexSet<T, S2>) -> Union<''a, T, S>
where
    S2: BuildHasher { /* ... */ }
  ```
  Return an iterator over all values that are in `self` or `other`.

- ```rust
  pub fn splice<R, I>(self: &mut Self, range: R, replace_with: I) -> Splice<''_, <I as >::IntoIter, T, S>
where
    R: RangeBounds<usize>,
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Creates a splicing iterator that replaces the specified range in the set

- ```rust
  pub fn append<S2>(self: &mut Self, other: &mut IndexSet<T, S2>) { /* ... */ }
  ```
  Moves all values from `other` into `self`, leaving `other` empty.

- ```rust
  pub fn contains<Q>(self: &Self, value: &Q) -> bool
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Return `true` if an equivalent to `value` exists in the set.

- ```rust
  pub fn get<Q>(self: &Self, value: &Q) -> Option<&T>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Return a reference to the value stored in the set, if it is present,

- ```rust
  pub fn get_full<Q>(self: &Self, value: &Q) -> Option<(usize, &T)>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Return item index and value

- ```rust
  pub fn get_index_of<Q>(self: &Self, value: &Q) -> Option<usize>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Return item index, if it exists in the set

- ```rust
  pub fn remove<Q>(self: &mut Self, value: &Q) -> bool
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Remove the value from the set, and return `true` if it was present.

- ```rust
  pub fn swap_remove<Q>(self: &mut Self, value: &Q) -> bool
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Remove the value from the set, and return `true` if it was present.

- ```rust
  pub fn shift_remove<Q>(self: &mut Self, value: &Q) -> bool
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Remove the value from the set, and return `true` if it was present.

- ```rust
  pub fn take<Q>(self: &mut Self, value: &Q) -> Option<T>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Removes and returns the value in the set, if any, that is equal to the

- ```rust
  pub fn swap_take<Q>(self: &mut Self, value: &Q) -> Option<T>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Removes and returns the value in the set, if any, that is equal to the

- ```rust
  pub fn shift_take<Q>(self: &mut Self, value: &Q) -> Option<T>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Removes and returns the value in the set, if any, that is equal to the

- ```rust
  pub fn swap_remove_full<Q>(self: &mut Self, value: &Q) -> Option<(usize, T)>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Remove the value from the set return it and the index it had.

- ```rust
  pub fn shift_remove_full<Q>(self: &mut Self, value: &Q) -> Option<(usize, T)>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
  ```
  Remove the value from the set return it and the index it had.

- ```rust
  pub fn pop(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Remove the last value

- ```rust
  pub fn retain<F>(self: &mut Self, keep: F)
where
    F: FnMut(&T) -> bool { /* ... */ }
  ```
  Scan through each value in the set and keep those where the

- ```rust
  pub fn sort(self: &mut Self)
where
    T: Ord { /* ... */ }
  ```
  Sort the set’s values by their default ordering.

- ```rust
  pub fn sort_by<F>(self: &mut Self, cmp: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the set’s values in place using the comparison function `cmp`.

- ```rust
  pub fn sorted_by<F>(self: Self, cmp: F) -> IntoIter<T>
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the values of the set and return a by-value iterator of

- ```rust
  pub fn sort_unstable(self: &mut Self)
where
    T: Ord { /* ... */ }
  ```
  Sort the set's values by their default ordering.

- ```rust
  pub fn sort_unstable_by<F>(self: &mut Self, cmp: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the set's values in place using the comparison function `cmp`.

- ```rust
  pub fn sorted_unstable_by<F>(self: Self, cmp: F) -> IntoIter<T>
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the values of the set and return a by-value iterator of

- ```rust
  pub fn sort_by_cached_key<K, F>(self: &mut Self, sort_key: F)
where
    K: Ord,
    F: FnMut(&T) -> K { /* ... */ }
  ```
  Sort the set’s values in place using a key extraction function.

- ```rust
  pub fn binary_search(self: &Self, x: &T) -> Result<usize, usize>
where
    T: Ord { /* ... */ }
  ```
  Search over a sorted set for a value.

- ```rust
  pub fn binary_search_by<''a, F>(self: &''a Self, f: F) -> Result<usize, usize>
where
    F: FnMut(&''a T) -> Ordering { /* ... */ }
  ```
  Search over a sorted set with a comparator function.

- ```rust
  pub fn binary_search_by_key<''a, B, F>(self: &''a Self, b: &B, f: F) -> Result<usize, usize>
where
    F: FnMut(&''a T) -> B,
    B: Ord { /* ... */ }
  ```
  Search over a sorted set with an extraction function.

- ```rust
  pub fn partition_point<P>(self: &Self, pred: P) -> usize
where
    P: FnMut(&T) -> bool { /* ... */ }
  ```
  Returns the index of the partition point of a sorted set according to the given predicate

- ```rust
  pub fn reverse(self: &mut Self) { /* ... */ }
  ```
  Reverses the order of the set’s values in place.

- ```rust
  pub fn as_slice(self: &Self) -> &Slice<T> { /* ... */ }
  ```
  Returns a slice of all the values in the set.

- ```rust
  pub fn into_boxed_slice(self: Self) -> Box<Slice<T>> { /* ... */ }
  ```
  Converts into a boxed slice of all the values in the set.

- ```rust
  pub fn get_index(self: &Self, index: usize) -> Option<&T> { /* ... */ }
  ```
  Get a value by index

- ```rust
  pub fn get_range<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<&Slice<T>> { /* ... */ }
  ```
  Returns a slice of values in the given range of indices.

- ```rust
  pub fn first(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Get the first value

- ```rust
  pub fn last(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Get the last value

- ```rust
  pub fn swap_remove_index(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Remove the value by index

- ```rust
  pub fn shift_remove_index(self: &mut Self, index: usize) -> Option<T> { /* ... */ }
  ```
  Remove the value by index

- ```rust
  pub fn move_index(self: &mut Self, from: usize, to: usize) { /* ... */ }
  ```
  Moves the position of a value from one index to another

- ```rust
  pub fn swap_indices(self: &mut Self, a: usize, b: usize) { /* ... */ }
  ```
  Swaps the position of two values in the set.

- ```rust
  pub fn is_disjoint<S2>(self: &Self, other: &IndexSet<T, S2>) -> bool
where
    S2: BuildHasher { /* ... */ }
  ```
  Returns `true` if `self` has no elements in common with `other`.

- ```rust
  pub fn is_subset<S2>(self: &Self, other: &IndexSet<T, S2>) -> bool
where
    S2: BuildHasher { /* ... */ }
  ```
  Returns `true` if all elements of `self` are contained in `other`.

- ```rust
  pub fn is_superset<S2>(self: &Self, other: &IndexSet<T, S2>) -> bool
where
    S2: BuildHasher { /* ... */ }
  ```
  Returns `true` if all elements of `other` are contained in `self`.

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Return an empty [`IndexSet`]

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: &IndexSet<T, S2>) -> <Self as >::Output { /* ... */ }
    ```
    Returns the set symmetric-difference, cloned into a new set.

- **MutableValues**
  - ```rust
    fn get_full_mut2<Q>(self: &mut Self, value: &Q) -> Option<(usize, &mut T)>
where
    Q: ?Sized + Hash + Equivalent<T> { /* ... */ }
    ```

  - ```rust
    fn get_index_mut2(self: &mut Self, index: usize) -> Option<&mut T> { /* ... */ }
    ```

  - ```rust
    fn retain2<F>(self: &mut Self, keep: F)
where
    F: FnMut(&mut T) -> bool { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: &IndexSet<T, S2>) -> <Self as >::Output { /* ... */ }
    ```
    Returns the set union, cloned into a new set.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Sub**
  - ```rust
    fn sub(self: Self, other: &IndexSet<T, S2>) -> <Self as >::Output { /* ... */ }
    ```
    Returns the set difference, cloned into a new set.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(arr: [T; N]) -> Self { /* ... */ }
    ```
    # Examples

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, other: &Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IndexSet<T, S2>) -> bool { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: &IndexSet<T, S2>) -> <Self as >::Output { /* ... */ }
    ```
    Returns the set intersection, cloned into a new set.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iterable: I) { /* ... */ }
    ```

  - ```rust
    fn extend<I: IntoIterator<Item = &''a T>>(self: &mut Self, iterable: I) { /* ... */ }
    ```

- **UnwindSafe**
- **Index**
  - ```rust
    fn index(self: &Self, range: ops::Range<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeFrom<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeFull) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeTo<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: ops::RangeToInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: (Bound<usize>, Bound<usize>)) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: usize) -> &T { /* ... */ }
    ```
    Returns a reference to the value at the supplied `index`.

### Re-exports

#### Re-export `Difference`

```rust
pub use self::iter::Difference;
```

#### Re-export `Drain`

```rust
pub use self::iter::Drain;
```

#### Re-export `Intersection`

```rust
pub use self::iter::Intersection;
```

#### Re-export `IntoIter`

```rust
pub use self::iter::IntoIter;
```

#### Re-export `Iter`

```rust
pub use self::iter::Iter;
```

#### Re-export `Splice`

```rust
pub use self::iter::Splice;
```

#### Re-export `SymmetricDifference`

```rust
pub use self::iter::SymmetricDifference;
```

#### Re-export `Union`

```rust
pub use self::iter::Union;
```

#### Re-export `MutableValues`

```rust
pub use self::mutable::MutableValues;
```

#### Re-export `Slice`

```rust
pub use self::slice::Slice;
```

## Types

### Struct `TryReserveError`

The error type for [`try_reserve`][IndexMap::try_reserve] methods.

```rust
pub struct TryReserveError {
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryReserveError { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryReserveError) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Enum `GetDisjointMutError`

The error type returned by [`get_disjoint_indices_mut`][`IndexMap::get_disjoint_indices_mut`].

It indicates one of two possible errors:
- An index is out-of-bounds.
- The same index appeared multiple times in the array.

```rust
pub enum GetDisjointMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}
```

#### Variants

##### `IndexOutOfBounds`

An index provided was out-of-bounds for the slice.

##### `OverlappingIndices`

Two indices provided were overlapping.

#### Implementations

##### Trait Implementations

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Error**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

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
    fn clone(self: &Self) -> GetDisjointMutError { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GetDisjointMutError) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Macros

### Macro `indexmap_with_default`

**Attributes:**

- `#[macro_export]`

Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs
and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.

## Example

```
use indexmap::indexmap_with_default;
use fnv::FnvHasher;

let map = indexmap_with_default!{
    FnvHasher;
    "a" => 1,
    "b" => 2,
};
assert_eq!(map["a"], 1);
assert_eq!(map["b"], 2);
assert_eq!(map.get("c"), None);

// "a" is the first key
assert_eq!(map.keys().next(), Some(&"a"));
```

```rust
pub macro_rules! indexmap_with_default {
    /* macro_rules! indexmap_with_default {
    ($H:ty; $($key:expr => $value:expr,)+) => { ... };
    ($H:ty; $($key:expr => $value:expr),*) => { ... };
} */
}
```

### Macro `indexmap`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[macro_export]`

Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs

## Example

```
use indexmap::indexmap;

let map = indexmap!{
    "a" => 1,
    "b" => 2,
};
assert_eq!(map["a"], 1);
assert_eq!(map["b"], 2);
assert_eq!(map.get("c"), None);

// "a" is the first key
assert_eq!(map.keys().next(), Some(&"a"));
```

```rust
pub macro_rules! indexmap {
    /* macro_rules! indexmap {
    ($($key:expr => $value:expr,)+) => { ... };
    ($($key:expr => $value:expr),*) => { ... };
} */
}
```

### Macro `indexset_with_default`

**Attributes:**

- `#[macro_export]`

Create an [`IndexSet`][crate::IndexSet] from a list of values
and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.

## Example

```
use indexmap::indexset_with_default;
use fnv::FnvHasher;

let set = indexset_with_default!{
    FnvHasher;
    "a",
    "b",
};
assert!(set.contains("a"));
assert!(set.contains("b"));
assert!(!set.contains("c"));

// "a" is the first value
assert_eq!(set.iter().next(), Some(&"a"));
```

```rust
pub macro_rules! indexset_with_default {
    /* macro_rules! indexset_with_default {
    ($H:ty; $($value:expr,)+) => { ... };
    ($H:ty; $($value:expr),*) => { ... };
} */
}
```

### Macro `indexset`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[macro_export]`

Create an [`IndexSet`][crate::IndexSet] from a list of values

## Example

```
use indexmap::indexset;

let set = indexset!{
    "a",
    "b",
};
assert!(set.contains("a"));
assert!(set.contains("b"));
assert!(!set.contains("c"));

// "a" is the first value
assert_eq!(set.iter().next(), Some(&"a"));
```

```rust
pub macro_rules! indexset {
    /* macro_rules! indexset {
    ($($value:expr,)+) => { ... };
    ($($value:expr),*) => { ... };
} */
}
```

## Re-exports

### Re-export `IndexMap`

```rust
pub use crate::map::IndexMap;
```

### Re-export `IndexSet`

```rust
pub use crate::set::IndexSet;
```

### Re-export `Equivalent`

```rust
pub use equivalent::Equivalent;
```

