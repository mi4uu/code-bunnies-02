# Crate Documentation

**Version:** 0.7.3

**Format Version:** 43

# Module `ordered_multimap`

This crate provides a type [`ListOrderedMultimap`] which is a multimap that maintains insertion order across all
keys and values.

See the type documentation for more information.

## Modules

## Module `list_ordered_multimap`

**Attributes:**

- `#[allow(unsafe_code)]`

Provides types related to the usage of [`ListOrderedMultimap`].

```rust
pub mod list_ordered_multimap { /* ... */ }
```

### Types

#### Type Alias `RandomState`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A random state to use for the hashmap in the multimap.

```rust
pub type RandomState = std::collections::hash_map::RandomState;
```

#### Struct `ListOrderedMultimap`

A multimap that associates with each key a list of values.

# Ordering

The primary guarantee this type gives is that regardless of what you do to the multimap, you are always able to
iterate through all keys and values in the order they were inserted. Values can be iterated by their insertion order
either for a specific key or for the entire map.

# Allocations

Allocations may be performed on any key-value insertion.

```rust
pub struct ListOrderedMultimap<Key, Value, State = RandomState> {
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
  pub fn new() -> ListOrderedMultimap<Key, Value, RandomState> { /* ... */ }
  ```
  Creates a new multimap with no initial capacity.

- ```rust
  pub fn with_capacity(key_capacity: usize, value_capacity: usize) -> ListOrderedMultimap<Key, Value, RandomState> { /* ... */ }
  ```
  Creates a new multimap with the specified capacities.

- ```rust
  pub fn with_capacity_and_hasher(key_capacity: usize, value_capacity: usize, state: State) -> ListOrderedMultimap<Key, Value, State> { /* ... */ }
  ```
  Creates a new multimap with the specified capacities and the given hash builder to hash keys.

- ```rust
  pub fn with_hasher(state: State) -> ListOrderedMultimap<Key, Value, State> { /* ... */ }
  ```
  Creates a new multimap with no capacity which will use the given hash builder to hash keys.

- ```rust
  pub fn back(self: &Self) -> Option<(&Key, &Value)> { /* ... */ }
  ```
  Returns an immutable reference to the first key-value pair in the multimap

- ```rust
  pub fn back_mut(self: &mut Self) -> Option<(&Key, &mut Value)> { /* ... */ }
  ```
  Returns an immutable reference to the first key-value pair in the multimap

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Removes all keys and values from the multimap.

- ```rust
  pub fn front(self: &Self) -> Option<(&Key, &Value)> { /* ... */ }
  ```
  Returns an immutable reference to the first key-value pair in the multimap

- ```rust
  pub fn front_mut(self: &mut Self) -> Option<(&Key, &mut Value)> { /* ... */ }
  ```
  Returns an immutable reference to the first key-value pair in the multimap

- ```rust
  pub fn hasher(self: &Self) -> &State { /* ... */ }
  ```
  Returns a reference to the multimap's [`BuildHasher`].

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the multimap is empty.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, Key, Value> { /* ... */ }
  ```
  Returns an iterator that yields immutable references to all key-value pairs in the multimap by insertion order.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, Key, Value> { /* ... */ }
  ```
  Returns an iterator that yields mutable references to all key-value pairs in the multimap by insertion order.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, Key> { /* ... */ }
  ```
  Returns an iterator that yields immutable references to all keys in the multimap by insertion order.

- ```rust
  pub fn keys_capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of keys the multimap can hold without reallocating.

- ```rust
  pub fn keys_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of keys in the multimap.

- ```rust
  pub fn pairs(self: &Self) -> KeyValues<''_, Key, Value, State> { /* ... */ }
  ```
  Returns an iterator that yields immutable references to keys and all associated values with those keys as separate

- ```rust
  pub fn pairs_mut(self: &mut Self) -> KeyValuesMut<''_, Key, Value, State> { /* ... */ }
  ```
  Returns an iterator that yields immutable references to keys and mutable references to all associated values with

- ```rust
  pub fn reserve_values(self: &mut Self, additional_capacity: usize) { /* ... */ }
  ```
  Reserves additional capacity such that more values can be stored in the multimap.

- ```rust
  pub fn values(self: &Self) -> Values<''_, Key, Value> { /* ... */ }
  ```
  Returns an iterator that yields immutable references to all values in the multimap by insertion order.

- ```rust
  pub fn values_mut(self: &mut Self) -> ValuesMut<''_, Key, Value> { /* ... */ }
  ```
  Returns an iterator that yields mutable references to all values in the multimap by insertion order.

- ```rust
  pub fn values_capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of values the multimap can hold without reallocating.

- ```rust
  pub fn values_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of values in the multimap across all keys.

- ```rust
  pub fn append(self: &mut Self, key: Key, value: Value) -> bool { /* ... */ }
  ```
  Appends a value to the list of values associated with the given key.

- ```rust
  pub fn contains_key<KeyQuery>(self: &Self, key: &KeyQuery) -> bool
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns whether the given key is in the multimap.

- ```rust
  pub fn entry(self: &mut Self, key: Key) -> Entry<''_, Key, Value, State> { /* ... */ }
  ```
  Returns whether the given key is in the multimap.

- ```rust
  pub fn entry_len<KeyQuery>(self: &Self, key: &KeyQuery) -> usize
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns the number of values associated with a key.

- ```rust
  pub fn get<KeyQuery>(self: &Self, key: &KeyQuery) -> Option<&Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns an immutable reference to the first value, by insertion order, associated with the given key, or `None` if

- ```rust
  pub fn get_all<KeyQuery>(self: &Self, key: &KeyQuery) -> EntryValues<''_, Key, Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns an iterator that yields immutable references to all values associated with the given key by insertion

- ```rust
  pub fn get_all_mut<KeyQuery>(self: &mut Self, key: &KeyQuery) -> EntryValuesMut<''_, Key, Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns an iterator that yields mutable references to all values associated with the given key by insertion order.

- ```rust
  pub fn get_mut<KeyQuery>(self: &mut Self, key: &KeyQuery) -> Option<&mut Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Returns a mutable reference to the first value, by insertion order, associated with the given key, or `None` if

- ```rust
  pub fn insert(self: &mut Self, key: Key, value: Value) -> Option<Value> { /* ... */ }
  ```
  Inserts the key-value pair into the multimap and returns the first value, by insertion order, that was already

- ```rust
  pub fn insert_all(self: &mut Self, key: Key, value: Value) -> EntryValuesDrain<''_, Key, Value> { /* ... */ }
  ```
  Inserts the key-value pair into the multimap and returns an iterator that yields all values previously associated

- ```rust
  pub fn pack_to(self: &mut Self, keys_minimum_capacity: usize, values_minimum_capacity: usize)
where
    State: Default { /* ... */ }
  ```
  Reorganizes the multimap to ensure maximum spatial locality and changes the key and value capacities to the

- ```rust
  pub fn pack_to_fit(self: &mut Self)
where
    State: Default { /* ... */ }
  ```
  Reorganizes the multimap to ensure maximum spatial locality and removes any excess key and value capacity.

- ```rust
  pub fn pop_back(self: &mut Self) -> Option<(KeyWrapper<''_, Key>, Value)> { /* ... */ }
  ```
  Removes the last key-value pair to have been inserted.

- ```rust
  pub fn pop_front(self: &mut Self) -> Option<(KeyWrapper<''_, Key>, Value)> { /* ... */ }
  ```
  Removes the first key-value pair to have been inserted.

- ```rust
  pub fn remove<KeyQuery>(self: &mut Self, key: &KeyQuery) -> Option<Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Removes all values associated with the given key from the map and returns the first value by insertion order.

- ```rust
  pub fn remove_all<KeyQuery>(self: &mut Self, key: &KeyQuery) -> EntryValuesDrain<''_, Key, Value>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Removes all values associated with the given key from the map and returns an iterator that yields those values.

- ```rust
  pub fn remove_entry<KeyQuery>(self: &mut Self, key: &KeyQuery) -> Option<(Key, Value)>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Removes all values associated with the given key from the map and returns the key and first value.

- ```rust
  pub fn remove_entry_all<KeyQuery>(self: &mut Self, key: &KeyQuery) -> Option<(Key, EntryValuesDrain<''_, Key, Value>)>
where
    Key: Borrow<KeyQuery>,
    KeyQuery: ?Sized + Eq + Hash { /* ... */ }
  ```
  Removes all values associated with the given key from the map and returns the key and an iterator that yields

- ```rust
  pub fn reserve_keys(self: &mut Self, additional_capacity: usize) { /* ... */ }
  ```
  Reserves additional capacity such that more keys can be stored in the multimap.

- ```rust
  pub fn retain<Function>(self: &mut Self, function: Function)
where
    Function: FnMut(&Key, &mut Value) -> bool { /* ... */ }
  ```
  Keeps all key-value pairs that satisfy the given predicate function.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<Iter>(self: &mut Self, iter: Iter)
where
    Iter: IntoIterator<Item = (Key, Value)> { /* ... */ }
    ```

  - ```rust
    fn extend<Iter>(self: &mut Self, iter: Iter)
where
    Iter: IntoIterator<Item = (&''a Key, &''a Value)> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ListOrderedMultimap<Key, Value, State>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ListOrderedMultimap<Key, Value, State> { /* ... */ }
    ```

- **Eq**
- **FromIterator**
  - ```rust
    fn from_iter<Iter>(iter: Iter) -> Self
where
    Iter: IntoIterator<Item = (Key, Value)> { /* ... */ }
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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Enum `KeyWrapper`

**Attributes:**

- `#[allow(single_use_lifetimes)]`

A wrapper around a key that is either borrowed or owned.

This type is similar to [`std::borrow::Cow`] but does not require a [`Clone`] trait bound on the key.

```rust
pub enum KeyWrapper<''map, Key> {
    Borrowed(&''map Key),
    Owned(Key),
}
```

##### Variants

###### `Borrowed`

An immutable reference to a key. This implies that the key is still associated to at least one value in the
multimap.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''map Key` |  |

###### `Owned`

An owned key. This will occur when a key is no longer associated with any values in the multimap.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Key` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_owned(self: Self) -> Key
where
    Key: Clone { /* ... */ }
  ```
  If the key wrapped is owned, it is returned. Otherwise, the borrowed key is cloned and returned.

- ```rust
  pub fn is_borrowed(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the wrapped key is borrowed.

- ```rust
  pub fn is_owned(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the wrapped key is owned.

###### Trait Implementations

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Eq**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &KeyWrapper<''map, Key>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> KeyWrapper<''map, Key> { /* ... */ }
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

#### Enum `Entry`

A view into a single entry in the multimap, which may either be vacant or occupied.

```rust
pub enum Entry<''map, Key, Value, State = RandomState> {
    Occupied(OccupiedEntry<''map, Key, Value>),
    Vacant(VacantEntry<''map, Key, Value, State>),
}
```

##### Variants

###### `Occupied`

An occupied entry associated with one or more values.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OccupiedEntry<''map, Key, Value>` |  |

###### `Vacant`

A vacant entry with no associated values.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `VacantEntry<''map, Key, Value, State>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn and_modify<Function>(self: Self, function: Function) -> Self
where
    Function: FnOnce(&mut Value) { /* ... */ }
  ```
  Calls the given function with a mutable reference to the first value of this entry, by insertion order, if it is

- ```rust
  pub fn or_insert(self: Self, value: Value) -> &''map mut Value { /* ... */ }
  ```
  If the entry is vacant, the given value will be inserted into it and a mutable reference to that value will be

- ```rust
  pub fn or_insert_entry(self: Self, value: Value) -> OccupiedEntry<''map, Key, Value> { /* ... */ }
  ```
  If the entry is vacant, the given value will be inserted into it and the new occupied entry will be returned.

- ```rust
  pub fn or_insert_with<Function>(self: Self, function: Function) -> &''map mut Value
where
    Function: FnOnce() -> Value { /* ... */ }
  ```
  If the entry is vacant, the value returned from the given function will be inserted into it and a mutable

- ```rust
  pub fn or_insert_with_entry<Function>(self: Self, function: Function) -> OccupiedEntry<''map, Key, Value>
where
    Function: FnOnce() -> Value { /* ... */ }
  ```
  If the entry is vacant, the value returned from the given function will be inserted into it and the new occupied

###### Trait Implementations

- **Send**
- **Unpin**
- **RefUnwindSafe**
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
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `OccupiedEntry`

A view into an occupied entry in the multimap.

```rust
pub struct OccupiedEntry<''map, Key, Value> {
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
  pub fn append(self: &mut Self, value: Value) { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn get(self: &Self) -> &Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn insert(self: &mut Self, value: Value) -> Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn insert_all(self: &mut Self, value: Value) -> EntryValuesDrain<''_, Key, Value> { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn into_mut(self: Self) -> &''map mut Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn iter(self: &Self) -> EntryValues<''_, Key, Value> { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn iter_mut(self: &mut Self) -> EntryValuesMut<''_, Key, Value> { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn key(self: &Self) -> &Key { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn remove(self: Self) -> Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn remove_all(self: Self) -> EntryValuesDrain<''map, Key, Value> { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn remove_entry(self: Self) -> (Key, Value) { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn remove_entry_all(self: Self) -> (Key, EntryValuesDrain<''map, Key, Value>) { /* ... */ }
  ```
  # Examples

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `VacantEntry`

A view into a vacant entry in the multimap.

```rust
pub struct VacantEntry<''map, Key, Value, State = RandomState> {
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
  pub fn insert(self: Self, value: Value) -> &''map mut Value { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn insert_entry(self: Self, value: Value) -> OccupiedEntry<''map, Key, Value> { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn into_key(self: Self) -> Key { /* ... */ }
  ```
  # Examples

- ```rust
  pub fn key(self: &Self) -> &Key { /* ... */ }
  ```
  # Examples

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
#### Struct `EntryValues`

An iterator that yields immutable references to all values of a given key. The order of the values is always in the
order that they were inserted.

```rust
pub struct EntryValues<''map, Key, Value> {
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
    fn clone(self: &Self) -> EntryValues<''map, Key, Value> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **FusedIterator**
- **ExactSizeIterator**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **Sync**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `EntryValuesDrain`

An iterator that moves all values of a given key out of a multimap but preserves the underlying capacity. The order
of the values is always in the order that they were inserted.

```rust
pub struct EntryValuesDrain<''map, Key, Value> {
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
  pub fn iter(self: &Self) -> EntryValues<''_, Key, Value> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to all values of a given key.

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FusedIterator**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ExactSizeIterator**
- **Sync**
#### Struct `EntryValuesMut`

An iterator that yields mutable references to all values of a given key. The order of the values is always in the
order that they were inserted.

```rust
pub struct EntryValuesMut<''map, Key, Value> {
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
  pub fn iter(self: &Self) -> EntryValues<''_, Key, Value> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to all values of a given key.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FusedIterator**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

#### Struct `IntoIter`

An iterator that owns and yields all key-value pairs in a multimap by cloning the keys for their possibly multiple
values. This is unnecessarily expensive whenever [`Iter`] or [`IterMut`] would suit as well. The order of the
yielded items is always in the order that they were inserted.

```rust
pub struct IntoIter<Key, Value> {
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
  pub fn iter(self: &Self) -> Iter<''_, Key, Value> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to all key-value pairs in a multimap.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Iter`

An iterator that yields immutable references to all key-value pairs in a multimap. The order of the yielded items is
always in the order that they were inserted.

```rust
pub struct Iter<''map, Key, Value> {
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

- **FusedIterator**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<''map, Key, Value> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **Sync**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
#### Struct `IterMut`

An iterator that yields mutable references to all key-value pairs in a multimap. The order of the yielded items is
always in the order that they were inserted.

```rust
pub struct IterMut<''map, Key, Value> {
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
  pub fn iter(self: &Self) -> Iter<''_, Key, Value> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to all key-value pairs in a multimap.

###### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FusedIterator**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **ExactSizeIterator**
- **Send**
#### Struct `KeyValues`

An iterator that yields immutable references to all keys and their value iterators. The order of the yielded items
is always in the order the keys were first inserted.

```rust
pub struct KeyValues<''map, Key, Value, State = RandomState> {
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> KeyValues<''map, Key, Value, State> { /* ... */ }
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

- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ExactSizeIterator**
#### Struct `KeyValuesMut`

An iterator that yields mutable references to all keys and their value iterators. The order of the yielded items is
always in the order the keys were first inserted.

```rust
pub struct KeyValuesMut<''map, Key, Value, State = RandomState> {
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
  pub fn iter(self: &Self) -> KeyValues<''_, Key, Value, State> { /* ... */ }
  ```
  Creates an iterator that yields mutable references to all key-value pairs of a multimap.

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
- **FusedIterator**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Keys`

An iterator that yields immutable references to all keys in the multimap. The order of the keys is always in the
order that they were first inserted.

```rust
pub struct Keys<''map, Key>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Keys<''map, Key> { /* ... */ }
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `Values`

An iterator that yields immutable references to all values of a multimap. The order of the values is always in the
order that they were inserted.

```rust
pub struct Values<''map, Key, Value>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **RefUnwindSafe**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **FusedIterator**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ExactSizeIterator**
- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Values<''map, Key, Value> { /* ... */ }
    ```

#### Struct `ValuesMut`

An iterator that yields mutable references to all values of a multimap. The order of the values is always in the
order that they were inserted.

```rust
pub struct ValuesMut<''map, Key, Value>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn iter(self: &Self) -> Values<''_, Key, Value> { /* ... */ }
  ```
  Creates an iterator that yields immutable references to all values of a multimap.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **FusedIterator**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **ExactSizeIterator**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `DummyHasher`

Dummy hasher that is not meant to be used. It is simply a placeholder.

```rust
pub struct DummyHasher;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DummyHasher { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Hasher**
  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn write(self: &mut Self, _: &[u8]) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `ListOrderedMultimap`

```rust
pub use self::list_ordered_multimap::ListOrderedMultimap;
```

