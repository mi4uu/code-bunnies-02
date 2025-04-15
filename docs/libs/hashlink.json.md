# Crate Documentation

**Version:** 0.10.0

**Format Version:** 43

# Module `hashlink`

## Modules

## Module `linked_hash_map`

```rust
pub mod linked_hash_map { /* ... */ }
```

### Types

#### Enum `TryReserveError`

```rust
pub enum TryReserveError {
    CapacityOverflow,
    AllocError {
        layout: core::alloc::Layout,
    },
}
```

##### Variants

###### `CapacityOverflow`

###### `AllocError`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `layout` | `core::alloc::Layout` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Sync**
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
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `LinkedHashMap`

A version of `HashMap` that has a user controllable order for its entries.

It achieves this by keeping its entries in an internal linked list and using a `HashMap` to
point at nodes in this linked list.

The order of entries defaults to "insertion order", but the user can also modify the order of
existing entries by manually moving them to the front or back.

There are two kinds of methods that modify the order of the internal list:

* Methods that have names like `to_front` and `to_back` will unsurprisingly move an existing
  entry to the front or back
* Methods that have the word `insert` will insert a new entry ot the back of the list, and if
  that method might replace an entry, that method will *also move that existing entry to the
  back*.

```rust
pub struct LinkedHashMap<K, V, S = crate::DefaultHashBuilder> {
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

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_hasher(hash_builder: S) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn values(self: &Self) -> Values<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn values_mut(self: &mut Self) -> ValuesMut<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn front(self: &Self) -> Option<(&K, &V)> { /* ... */ }
  ```

- ```rust
  pub fn back(self: &Self) -> Option<(&K, &V)> { /* ... */ }
  ```

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&K, &mut V) -> bool { /* ... */ }
  ```

- ```rust
  pub fn hasher(self: &Self) -> &S { /* ... */ }
  ```

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn entry(self: &mut Self, key: K) -> Entry<''_, K, V, S> { /* ... */ }
  ```

- ```rust
  pub fn get<Q>(self: &Self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn get_key_value<Q>(self: &Self, k: &Q) -> Option<(&K, &V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn contains_key<Q>(self: &Self, k: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn get_mut<Q>(self: &mut Self, k: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, k: K, v: V) -> Option<V> { /* ... */ }
  ```
  Inserts the given key / value pair at the *back* of the internal linked list.

- ```rust
  pub fn replace(self: &mut Self, k: K, v: V) -> Option<V> { /* ... */ }
  ```
  If the given key is not in this map, inserts the key / value pair at the *back* of the

- ```rust
  pub fn remove<Q>(self: &mut Self, k: &Q) -> Option<V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn remove_entry<Q>(self: &mut Self, k: &Q) -> Option<(K, V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<(K, V)> { /* ... */ }
  ```

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<(K, V)> { /* ... */ }
  ```

- ```rust
  pub fn to_front<Q>(self: &mut Self, k: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  If an entry with this key exists, move it to the front of the list and return a reference to

- ```rust
  pub fn to_back<Q>(self: &mut Self, k: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  If an entry with this key exists, move it to the back of the list and return a reference to

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn retain_with_order<F>(self: &mut Self, f: F)
where
    F: FnMut(&K, &mut V) -> bool { /* ... */ }
  ```

- ```rust
  pub fn cursor_front_mut(self: &mut Self) -> CursorMut<''_, K, V, S> { /* ... */ }
  ```
  Returns the `CursorMut` over the front node.

- ```rust
  pub fn cursor_back_mut(self: &mut Self) -> CursorMut<''_, K, V, S> { /* ... */ }
  ```
  Returns the `CursorMut` over the back node.

- ```rust
  pub fn raw_entry(self: &Self) -> RawEntryBuilder<''_, K, V, S> { /* ... */ }
  ```

- ```rust
  pub fn raw_entry_mut(self: &mut Self) -> RawEntryBuilderMut<''_, K, V, S> { /* ... */ }
  ```

###### Trait Implementations

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Freeze**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: &''a Q) -> &mut V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, h: &mut H) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = (K, V)>>(self: &mut Self, iter: I) { /* ... */ }
    ```

  - ```rust
    fn extend<I: IntoIterator<Item = (&''a K, &''a V)>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Index**
  - ```rust
    fn index(self: &Self, index: &''a Q) -> &V { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a, K, V> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, K, V> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IntoIter<K, V> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `Entry`

```rust
pub enum Entry<''a, K, V, S> {
    Occupied(OccupiedEntry<''a, K, V, S>),
    Vacant(VacantEntry<''a, K, V, S>),
}
```

##### Variants

###### `Occupied`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OccupiedEntry<''a, K, V, S>` |  |

###### `Vacant`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `VacantEntry<''a, K, V, S>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn or_insert(self: Self, default: V) -> &''a mut V
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```
  If this entry is vacant, inserts a new entry with the given value and returns a reference to

- ```rust
  pub fn or_insert_with<F: FnOnce() -> V>(self: Self, default: F) -> &''a mut V
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```
  Similar to `Entry::or_insert`, but accepts a function to construct a new value if this entry

- ```rust
  pub fn key(self: &Self) -> &K { /* ... */ }
  ```

- ```rust
  pub fn and_modify<F>(self: Self, f: F) -> Self
where
    F: FnOnce(&mut V) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `OccupiedEntry`

```rust
pub struct OccupiedEntry<''a, K, V, S> {
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
  pub fn key(self: &Self) -> &K { /* ... */ }
  ```

- ```rust
  pub fn remove_entry(self: Self) -> (K, V) { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self) -> &V { /* ... */ }
  ```

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut V { /* ... */ }
  ```

- ```rust
  pub fn into_mut(self: Self) -> &''a mut V { /* ... */ }
  ```

- ```rust
  pub fn to_back(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn to_front(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, value: V) -> V { /* ... */ }
  ```
  Replaces this entry's value with the provided value.

- ```rust
  pub fn remove(self: Self) -> V { /* ... */ }
  ```

- ```rust
  pub fn insert_entry(self: Self, value: V) -> (K, V) { /* ... */ }
  ```
  Similar to `OccupiedEntry::replace_entry`, but *does* move the entry to the back of the

- ```rust
  pub fn cursor_mut(self: Self) -> CursorMut<''a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher { /* ... */ }
  ```
  Returns a `CursorMut` over the current entry.

- ```rust
  pub fn replace_entry(self: Self, value: V) -> (K, V) { /* ... */ }
  ```
  Replaces the entry's key with the key provided to `LinkedHashMap::entry`, and replaces the

- ```rust
  pub fn replace_key(self: Self) -> K { /* ... */ }
  ```
  Replaces this entry's key with the key provided to `LinkedHashMap::entry`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
#### Struct `VacantEntry`

```rust
pub struct VacantEntry<''a, K, V, S> {
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
  pub fn key(self: &Self) -> &K { /* ... */ }
  ```

- ```rust
  pub fn into_key(self: Self) -> K { /* ... */ }
  ```

- ```rust
  pub fn insert(self: Self, value: V) -> &''a mut V
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```
  Insert's the key for this vacant entry paired with the given value as a new entry at the

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `RawEntryBuilder`

```rust
pub struct RawEntryBuilder<''a, K, V, S> {
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
  pub fn from_key<Q>(self: Self, k: &Q) -> Option<(&''a K, &''a V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn from_key_hashed_nocheck<Q>(self: Self, hash: u64, k: &Q) -> Option<(&''a K, &''a V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn from_hash</* synthetic */ impl FnMut(&K) -> bool: FnMut(&K) -> bool>(self: Self, hash: u64, is_match: impl FnMut(&K) -> bool) -> Option<(&''a K, &''a V)> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `RawEntryBuilderMut`

```rust
pub struct RawEntryBuilderMut<''a, K, V, S> {
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
  pub fn from_key<Q>(self: Self, k: &Q) -> RawEntryMut<''a, K, V, S>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn from_key_hashed_nocheck<Q>(self: Self, hash: u64, k: &Q) -> RawEntryMut<''a, K, V, S>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn from_hash</* synthetic */ impl FnMut(&K) -> bool: FnMut(&K) -> bool>(self: Self, hash: u64, is_match: impl FnMut(&K) -> bool) -> RawEntryMut<''a, K, V, S> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

#### Enum `RawEntryMut`

```rust
pub enum RawEntryMut<''a, K, V, S> {
    Occupied(RawOccupiedEntryMut<''a, K, V, S>),
    Vacant(RawVacantEntryMut<''a, K, V, S>),
}
```

##### Variants

###### `Occupied`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `RawOccupiedEntryMut<''a, K, V, S>` |  |

###### `Vacant`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `RawVacantEntryMut<''a, K, V, S>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn or_insert(self: Self, default_key: K, default_val: V) -> (&''a mut K, &''a mut V)
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```
  Similarly to `Entry::or_insert`, if this entry is occupied, it will move the existing entry

- ```rust
  pub fn or_insert_with<F>(self: Self, default: F) -> (&''a mut K, &''a mut V)
where
    F: FnOnce() -> (K, V),
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```
  Similarly to `Entry::or_insert_with`, if this entry is occupied, it will move the existing

- ```rust
  pub fn and_modify<F>(self: Self, f: F) -> Self
where
    F: FnOnce(&mut K, &mut V) { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `RawOccupiedEntryMut`

```rust
pub struct RawOccupiedEntryMut<''a, K, V, S> {
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
  pub fn key(self: &Self) -> &K { /* ... */ }
  ```

- ```rust
  pub fn key_mut(self: &mut Self) -> &mut K { /* ... */ }
  ```

- ```rust
  pub fn into_key(self: Self) -> &''a mut K { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self) -> &V { /* ... */ }
  ```

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut V { /* ... */ }
  ```

- ```rust
  pub fn into_mut(self: Self) -> &''a mut V { /* ... */ }
  ```

- ```rust
  pub fn get_key_value(self: &Self) -> (&K, &V) { /* ... */ }
  ```

- ```rust
  pub fn get_key_value_mut(self: &mut Self) -> (&mut K, &mut V) { /* ... */ }
  ```

- ```rust
  pub fn into_key_value(self: Self) -> (&''a mut K, &''a mut V) { /* ... */ }
  ```

- ```rust
  pub fn to_back(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn to_front(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn replace_value(self: &mut Self, value: V) -> V { /* ... */ }
  ```

- ```rust
  pub fn replace_key(self: &mut Self, key: K) -> K { /* ... */ }
  ```

- ```rust
  pub fn remove(self: Self) -> V { /* ... */ }
  ```

- ```rust
  pub fn remove_entry(self: Self) -> (K, V) { /* ... */ }
  ```

- ```rust
  pub fn cursor_mut(self: Self) -> CursorMut<''a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher { /* ... */ }
  ```
  Returns a `CursorMut` over the current entry.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `RawVacantEntryMut`

```rust
pub struct RawVacantEntryMut<''a, K, V, S> {
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
  pub fn insert(self: Self, key: K, value: V) -> (&''a mut K, &''a mut V)
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```

- ```rust
  pub fn insert_hashed_nocheck(self: Self, hash: u64, key: K, value: V) -> (&''a mut K, &''a mut V)
where
    K: Hash,
    S: BuildHasher { /* ... */ }
  ```

- ```rust
  pub fn insert_with_hasher</* synthetic */ impl Fn(&K) -> u64: Fn(&K) -> u64>(self: Self, hash: u64, key: K, value: V, hasher: impl Fn(&K) -> u64) -> (&''a mut K, &''a mut V)
where
    S: BuildHasher { /* ... */ }
  ```

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Iter`

```rust
pub struct Iter<''a, K, V> {
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(&''a K, &''a V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ExactSizeIterator**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Unpin**
- **Sync**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(&''a K, &''a V)> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `IterMut`

```rust
pub struct IterMut<''a, K, V> {
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
- **Unpin**
- **Send**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(&''a K, &''a mut V)> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(&''a K, &''a mut V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ExactSizeIterator**
- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
#### Struct `IntoIter`

```rust
pub struct IntoIter<K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(K, V)> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(K, V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `Drain`

```rust
pub struct Drain<''a, K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(K, V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(K, V)> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
#### Struct `CursorMut`

The `CursorMut` struct and its implementation provide the basic mutable Cursor API for Linked
lists as proposed in
[here](https://github.com/rust-lang/rfcs/blob/master/text/2570-linked-list-cursors.md), with
several exceptions:

- It behaves similarly to Rust's Iterators, returning `None` when the end of the list is
  reached. A _guard_ node is positioned between the head and tail of the linked list to
  facilitate this. If the cursor is over this guard node, `None` is returned, signaling the end
  or start of the list. From this position, the cursor can move in either direction as the
  linked list is circular, with the guard node connecting the two ends.
- The current implementation does not include an `index` method, as it does not track the index
  of its elements. It provides access to each map entry as a tuple of `(&K, &mut V)`.


```rust
pub struct CursorMut<''a, K, V, S> {
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
  pub fn current(self: &mut Self) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Returns an `Option` of the current element in the list, provided it is not the

- ```rust
  pub fn peek_next(self: &mut Self) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Retrieves the next element in the list (moving towards the end).

- ```rust
  pub fn peek_prev(self: &mut Self) -> Option<(&K, &mut V)> { /* ... */ }
  ```
  Retrieves the previous element in the list (moving towards the front).

- ```rust
  pub fn move_next(self: &mut Self) { /* ... */ }
  ```
  Updates the pointer to the current element to the next element in the

- ```rust
  pub fn move_prev(self: &mut Self) { /* ... */ }
  ```
  Updates the pointer to the current element to the previous element in the

- ```rust
  pub fn insert_before(self: &mut Self, key: K, value: V) -> Option<V>
where
    K: Eq + Hash,
    S: BuildHasher { /* ... */ }
  ```
  Inserts the provided key and value before the current element. It checks if an entry

- ```rust
  pub fn insert_after(self: &mut Self, key: K, value: V) -> Option<V>
where
    K: Eq + Hash,
    S: BuildHasher { /* ... */ }
  ```
  Inserts the provided key and value after the current element. It checks if an entry

###### Trait Implementations

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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
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

#### Struct `Keys`

```rust
pub struct Keys<''a, K, V> {
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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a K> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Keys<''a, K, V> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a K> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Values`

```rust
pub struct Values<''a, K, V> {
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
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
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

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a V> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a V> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
#### Struct `ValuesMut`

```rust
pub struct ValuesMut<''a, K, V> {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a mut V> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

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

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a mut V> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

## Module `linked_hash_set`

```rust
pub mod linked_hash_set { /* ... */ }
```

### Types

#### Struct `LinkedHashSet`

```rust
pub struct LinkedHashSet<T, S = crate::DefaultHashBuilder> {
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
  pub fn new() -> LinkedHashSet<T, DefaultHashBuilder> { /* ... */ }
  ```

- ```rust
  pub fn with_capacity(capacity: usize) -> LinkedHashSet<T, DefaultHashBuilder> { /* ... */ }
  ```

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&T) -> bool { /* ... */ }
  ```

- ```rust
  pub fn with_hasher(hasher: S) -> LinkedHashSet<T, S> { /* ... */ }
  ```

- ```rust
  pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> LinkedHashSet<T, S> { /* ... */ }
  ```

- ```rust
  pub fn hasher(self: &Self) -> &S { /* ... */ }
  ```

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), TryReserveError> { /* ... */ }
  ```

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn difference<''a>(self: &''a Self, other: &''a LinkedHashSet<T, S>) -> Difference<''a, T, S> { /* ... */ }
  ```

- ```rust
  pub fn symmetric_difference<''a>(self: &''a Self, other: &''a LinkedHashSet<T, S>) -> SymmetricDifference<''a, T, S> { /* ... */ }
  ```

- ```rust
  pub fn intersection<''a>(self: &''a Self, other: &''a LinkedHashSet<T, S>) -> Intersection<''a, T, S> { /* ... */ }
  ```

- ```rust
  pub fn union<''a>(self: &''a Self, other: &''a LinkedHashSet<T, S>) -> Union<''a, T, S> { /* ... */ }
  ```

- ```rust
  pub fn contains<Q>(self: &Self, value: &Q) -> bool
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn get<Q>(self: &Self, value: &Q) -> Option<&T>
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn get_or_insert(self: &mut Self, value: T) -> &T { /* ... */ }
  ```

- ```rust
  pub fn get_or_insert_with<Q, F>(self: &mut Self, value: &Q, f: F) -> &T
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    F: FnOnce(&Q) -> T { /* ... */ }
  ```

- ```rust
  pub fn is_disjoint(self: &Self, other: &LinkedHashSet<T, S>) -> bool { /* ... */ }
  ```

- ```rust
  pub fn is_subset(self: &Self, other: &LinkedHashSet<T, S>) -> bool { /* ... */ }
  ```

- ```rust
  pub fn is_superset(self: &Self, other: &LinkedHashSet<T, S>) -> bool { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, value: T) -> bool { /* ... */ }
  ```
  Inserts the given value into the set.

- ```rust
  pub fn replace(self: &mut Self, value: T) -> Option<T> { /* ... */ }
  ```
  Adds the given value to the set, replacing the existing value.

- ```rust
  pub fn remove<Q>(self: &mut Self, value: &Q) -> bool
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn take<Q>(self: &mut Self, value: &Q) -> Option<T>
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn front(self: &Self) -> Option<&T> { /* ... */ }
  ```

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<T> { /* ... */ }
  ```

- ```rust
  pub fn back(self: &Self) -> Option<&T> { /* ... */ }
  ```

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<T> { /* ... */ }
  ```

- ```rust
  pub fn to_front<Q>(self: &mut Self, value: &Q) -> bool
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn to_back<Q>(self: &mut Self, value: &Q) -> bool
where
    T: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn retain_with_order<F>(self: &mut Self, f: F)
where
    F: FnMut(&T) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: &LinkedHashSet<T, S>) -> LinkedHashSet<T, S> { /* ... */ }
    ```

- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> LinkedHashSet<T, S> { /* ... */ }
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

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> LinkedHashSet<T, S> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I) { /* ... */ }
    ```

  - ```rust
    fn extend<I: IntoIterator<Item = &''a T>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: &LinkedHashSet<T, S>) -> LinkedHashSet<T, S> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: &LinkedHashSet<T, S>) -> LinkedHashSet<T, S> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: &LinkedHashSet<T, S>) -> LinkedHashSet<T, S> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Iter`

```rust
pub struct Iter<''a, K> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<''a, K> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ExactSizeIterator**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a K> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IntoIter`

```rust
pub struct IntoIter<K> {
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

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<K> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **ExactSizeIterator**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<K> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Drain`

```rust
pub struct Drain<''a, K: ''a> {
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<K> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
- **Unpin**
- **UnwindSafe**
- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<K> { /* ... */ }
    ```

#### Struct `Intersection`

```rust
pub struct Intersection<''a, T, S> {
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
- **RefUnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Sync**
- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Intersection<''a, T, S> { /* ... */ }
    ```

#### Struct `Difference`

```rust
pub struct Difference<''a, T, S> {
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Difference<''a, T, S> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
#### Struct `SymmetricDifference`

```rust
pub struct SymmetricDifference<''a, T, S> {
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SymmetricDifference<''a, T, S> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
#### Struct `Union`

```rust
pub struct Union<''a, T, S> {
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

- **Send**
- **Freeze**
- **Sync**
- **Unpin**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Union<''a, T, S> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `lru_cache`

```rust
pub mod lru_cache { /* ... */ }
```

### Types

#### Struct `LruCache`

```rust
pub struct LruCache<K, V, S = crate::DefaultHashBuilder> {
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
  pub fn new(capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_unbounded() -> Self { /* ... */ }
  ```
  Create a new unbounded `LruCache` that does not automatically evict entries.

- ```rust
  pub fn with_hasher(capacity: usize, hash_builder: S) -> Self { /* ... */ }
  ```

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, K, V> { /* ... */ }
  ```

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, k: K, v: V) -> Option<V> { /* ... */ }
  ```
  Insert a new value into the `LruCache`.

- ```rust
  pub fn peek<Q>(self: &Self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Get the value for the given key, *without* marking the value as recently used and moving it

- ```rust
  pub fn peek_mut<Q>(self: &mut Self, k: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Get the value for the given key mutably, *without* marking the value as recently used and

- ```rust
  pub fn get<Q>(self: &mut Self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Retrieve the given key, marking it as recently used and moving it to the back of the LRU

- ```rust
  pub fn get_mut<Q>(self: &mut Self, k: &Q) -> Option<&mut V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Retrieve the given key, marking it as recently used and moving it to the back of the LRU

- ```rust
  pub fn entry(self: &mut Self, key: K) -> Entry<''_, K, V, S> { /* ... */ }
  ```
  If the returned entry is vacant, it will always have room to insert a single value.  By

- ```rust
  pub fn raw_entry(self: &Self) -> RawEntryBuilder<''_, K, V, S> { /* ... */ }
  ```
  The constructed raw entry is never automatically moved to the back of the LRU list.  By

- ```rust
  pub fn raw_entry_mut(self: &mut Self) -> RawEntryBuilderMut<''_, K, V, S> { /* ... */ }
  ```
  If the constructed raw entry is vacant, it will always have room to insert a single value.

- ```rust
  pub fn remove<Q>(self: &mut Self, k: &Q) -> Option<V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn remove_entry<Q>(self: &mut Self, k: &Q) -> Option<(K, V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```

- ```rust
  pub fn set_capacity(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Set the new cache capacity for the `LruCache`.

- ```rust
  pub fn remove_lru(self: &mut Self) -> Option<(K, V)> { /* ... */ }
  ```
  Remove the least recently used entry and return it.

###### Trait Implementations

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<K, V> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> Iter<''a, K, V> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, K, V> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = (K, V)>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `Drain`

```rust
pub use crate::linked_hash_map::Drain;
```

#### Re-export `Entry`

```rust
pub use crate::linked_hash_map::Entry;
```

#### Re-export `IntoIter`

```rust
pub use crate::linked_hash_map::IntoIter;
```

#### Re-export `Iter`

```rust
pub use crate::linked_hash_map::Iter;
```

#### Re-export `IterMut`

```rust
pub use crate::linked_hash_map::IterMut;
```

#### Re-export `OccupiedEntry`

```rust
pub use crate::linked_hash_map::OccupiedEntry;
```

#### Re-export `RawEntryBuilder`

```rust
pub use crate::linked_hash_map::RawEntryBuilder;
```

#### Re-export `RawEntryBuilderMut`

```rust
pub use crate::linked_hash_map::RawEntryBuilderMut;
```

#### Re-export `RawOccupiedEntryMut`

```rust
pub use crate::linked_hash_map::RawOccupiedEntryMut;
```

#### Re-export `RawVacantEntryMut`

```rust
pub use crate::linked_hash_map::RawVacantEntryMut;
```

#### Re-export `VacantEntry`

```rust
pub use crate::linked_hash_map::VacantEntry;
```

## Types

### Struct `DefaultHashBuilder`

Default hash builder, matches hashbrown's default hasher.

See [`DefaultHasher`] for more details.

```rust
pub struct DefaultHashBuilder(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Sync**
- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> <Self as >::Hasher { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> DefaultHashBuilder { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DefaultHashBuilder { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `DefaultHasher`

Default hasher, as selected by hashbrown.

```rust
pub struct DefaultHasher(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Hasher**
  - ```rust
    fn write(self: &mut Self, bytes: &[u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u8(self: &mut Self, i: u8) { /* ... */ }
    ```

  - ```rust
    fn write_u16(self: &mut Self, i: u16) { /* ... */ }
    ```

  - ```rust
    fn write_u32(self: &mut Self, i: u32) { /* ... */ }
    ```

  - ```rust
    fn write_u64(self: &mut Self, i: u64) { /* ... */ }
    ```

  - ```rust
    fn write_u128(self: &mut Self, i: u128) { /* ... */ }
    ```

  - ```rust
    fn write_usize(self: &mut Self, i: usize) { /* ... */ }
    ```

  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn write_i8(self: &mut Self, i: i8) { /* ... */ }
    ```

  - ```rust
    fn write_i16(self: &mut Self, i: i16) { /* ... */ }
    ```

  - ```rust
    fn write_i32(self: &mut Self, i: i32) { /* ... */ }
    ```

  - ```rust
    fn write_i64(self: &mut Self, i: i64) { /* ... */ }
    ```

  - ```rust
    fn write_i128(self: &mut Self, i: i128) { /* ... */ }
    ```

  - ```rust
    fn write_isize(self: &mut Self, i: isize) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DefaultHasher { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Re-exports

### Re-export `LinkedHashMap`

```rust
pub use linked_hash_map::LinkedHashMap;
```

### Re-export `LinkedHashSet`

```rust
pub use linked_hash_set::LinkedHashSet;
```

### Re-export `LruCache`

```rust
pub use lru_cache::LruCache;
```

