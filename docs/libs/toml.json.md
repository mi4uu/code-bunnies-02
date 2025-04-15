# Crate Documentation

**Version:** 0.8.20

**Format Version:** 43

# Module `toml`

A [serde]-compatible [TOML]-parsing library

TOML itself is a simple, ergonomic, and readable configuration format:

```toml
[package]
name = "toml"

[dependencies]
serde = "1.0"
```

The TOML format tends to be relatively common throughout the Rust community
for configuration, notably being used by [Cargo], Rust's package manager.

## TOML values

A TOML document is represented with the [`Table`] type which maps `String` to the [`Value`] enum:

```rust
# use toml::value::{Datetime, Array, Table};
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime),
    Array(Array),
    Table(Table),
}
```

## Parsing TOML

The easiest way to parse a TOML document is via the [`Table`] type:

 ```
use toml::Table;

let value = "foo = 'bar'".parse::<Table>().unwrap();

assert_eq!(value["foo"].as_str(), Some("bar"));
```

The [`Table`] type implements a number of convenience methods and
traits; the example above uses [`FromStr`] to parse a [`str`] into a
[`Table`].

## Deserialization and Serialization

This crate supports [`serde`] 1.0 with a number of
implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
`Serializer` traits. Namely, you'll find:

* `Deserialize for Table`
* `Serialize for Table`
* `Deserialize for Value`
* `Serialize for Value`
* `Deserialize for Datetime`
* `Serialize for Datetime`
* `Deserializer for de::Deserializer`
* `Serializer for ser::Serializer`
* `Deserializer for Table`
* `Deserializer for Value`

This means that you can use Serde to deserialize/serialize the
[`Table`] type as well as [`Value`] and [`Datetime`] type in this crate. You can also
use the [`Deserializer`], [`Serializer`], or [`Table`] type itself to act as
a deserializer/serializer for arbitrary types.

An example of deserializing with TOML is:

 ```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config: Config = toml::from_str(r#"
    ip = '127.0.0.1'

    [keys]
    github = 'xxxxxxxxxxxxxxxxx'
    travis = 'yyyyyyyyyyyyyyyyy'
"#).unwrap();

assert_eq!(config.ip, "127.0.0.1");
assert_eq!(config.port, None);
assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
```

You can serialize types in a similar fashion:

 ```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config = Config {
    ip: "127.0.0.1".to_string(),
    port: None,
    keys: Keys {
        github: "xxxxxxxxxxxxxxxxx".to_string(),
        travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
    },
};

let toml = toml::to_string(&config).unwrap();
```

[TOML]: https://github.com/toml-lang/toml
[Cargo]: https://crates.io/
[`serde`]: https://serde.rs/
[serde]: https://serde.rs/

## Modules

## Module `map`

A map of `String` to [Value].

By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
feature of toml-rs to use [`IndexMap`] instead.

[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[`IndexMap`]: https://docs.rs/indexmap

```rust
pub mod map { /* ... */ }
```

### Types

#### Struct `Map`

Represents a TOML key/value type.

```rust
pub struct Map<K, V> {
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
  Makes a new empty Map.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Makes a new empty Map with the given initial capacity.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the map, removing all values.

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash + ?Sized { /* ... */ }
  ```
  Returns a reference to the value corresponding to the key.

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash + ?Sized { /* ... */ }
  ```
  Returns true if the map contains a value for the specified key.

- ```rust
  pub fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the key.

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Returns the key-value pair matching the given key.

- ```rust
  pub fn insert(self: &mut Self, k: String, v: Value) -> Option<Value> { /* ... */ }
  ```
  Inserts a key-value pair into the map.

- ```rust
  pub fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash + ?Sized { /* ... */ }
  ```
  Removes a key from the map, returning the value at the key if the key

- ```rust
  pub fn retain<F>(self: &mut Self, keep: F)
where
    F: FnMut(&str, &mut Value) -> bool { /* ... */ }
  ```
  Retains only the elements specified by the `keep` predicate.

- ```rust
  pub fn entry<S>(self: &mut Self, key: S) -> Entry<''_>
where
    S: Into<String> { /* ... */ }
  ```
  Gets the given key's corresponding entry in the map for in-place

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the map contains no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Gets an iterator over the entries of the map.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_> { /* ... */ }
  ```
  Gets a mutable iterator over the entries of the map.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_> { /* ... */ }
  ```
  Gets an iterator over the keys of the map.

- ```rust
  pub fn values(self: &Self) -> Values<''_> { /* ... */ }
  ```
  Gets an iterator over the values of the map.

- ```rust
  pub fn try_from<T>(value: T) -> Result<Self, crate::ser::Error>
where
    T: ser::Serialize { /* ... */ }
  ```
  Convert a `T` into `toml::Table`.

- ```rust
  pub fn try_into<''de, T>(self: Self) -> Result<T, crate::de::Error>
where
    T: de::Deserialize<''de> { /* ... */ }
  ```
  Interpret a `toml::Table` as an instance of type `T`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T>(iter: T) -> Self
where
    T: IntoIterator<Item = (String, Value)> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: Table) -> Value { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: &Q) -> &mut Value { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T>(self: &mut Self, iter: T)
where
    T: IntoIterator<Item = (String, Value)> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **DeserializeOwned**
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

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: de::Deserializer<''de> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: &Q) -> &Value { /* ... */ }
    ```

- **Send**
#### Enum `Entry`

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the [`entry`] method on [`Map`].

[`entry`]: struct.Map.html#method.entry
[`Map`]: struct.Map.html

```rust
pub enum Entry<''a> {
    Vacant(VacantEntry<''a>),
    Occupied(OccupiedEntry<''a>),
}
```

##### Variants

###### `Vacant`

A vacant Entry.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `VacantEntry<''a>` |  |

###### `Occupied`

An occupied Entry.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OccupiedEntry<''a>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Returns a reference to this entry's key.

- ```rust
  pub fn or_insert(self: Self, default: Value) -> &''a mut Value { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the default if empty, and

- ```rust
  pub fn or_insert_with<F>(self: Self, default: F) -> &''a mut Value
where
    F: FnOnce() -> Value { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the result of the default

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Sync**
- **UnwindSafe**
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `VacantEntry`

A vacant Entry. It is part of the [`Entry`] enum.

[`Entry`]: enum.Entry.html

```rust
pub struct VacantEntry<''a> {
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
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Gets a reference to the key that would be used when inserting a value

- ```rust
  pub fn insert(self: Self, value: Value) -> &''a mut Value { /* ... */ }
  ```
  Sets the value of the entry with the `VacantEntry`'s key, and returns a

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `OccupiedEntry`

An occupied Entry. It is part of the [`Entry`] enum.

[`Entry`]: enum.Entry.html

```rust
pub struct OccupiedEntry<''a> {
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
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Gets a reference to the key in the entry.

- ```rust
  pub fn get(self: &Self) -> &Value { /* ... */ }
  ```
  Gets a reference to the value in the entry.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Value { /* ... */ }
  ```
  Gets a mutable reference to the value in the entry.

- ```rust
  pub fn into_mut(self: Self) -> &''a mut Value { /* ... */ }
  ```
  Converts the entry into a mutable reference to its value.

- ```rust
  pub fn insert(self: &mut Self, value: Value) -> Value { /* ... */ }
  ```
  Sets the value of the entry with the `OccupiedEntry`'s key, and returns

- ```rust
  pub fn remove(self: Self) -> Value { /* ... */ }
  ```
  Takes the value of the entry out of the map, and returns it.

###### Trait Implementations

- **Unpin**
- **Freeze**
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

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Iter`

An iterator over a `toml::Map`'s entries.

```rust
pub struct Iter<''a> {
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `IterMut`

A mutable iterator over a `toml::Map`'s entries.

```rust
pub struct IterMut<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Sync**
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
- **RefUnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `IntoIter`

An owning iterator over a `toml::Map`'s entries.

```rust
pub struct IntoIter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Freeze**
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

#### Struct `Keys`

An iterator over a `toml::Map`'s keys.

```rust
pub struct Keys<''a> {
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

- **Freeze**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **UnwindSafe**
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

#### Struct `Values`

An iterator over a `toml::Map`'s values.

```rust
pub struct Values<''a> {
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

- **Sync**
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
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `value`

Definition of a TOML [value][Value]

```rust
pub mod value { /* ... */ }
```

### Types

#### Type Alias `Array`

Type representing a TOML array, payload of the `Value::Array` variant

```rust
pub type Array = Vec<Value>;
```

#### Enum `Value`

Representation of a TOML value.

```rust
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime),
    Array(Array),
    Table(Table),
}
```

##### Variants

###### `String`

Represents a TOML string

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Integer`

Represents a TOML integer

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Float`

Represents a TOML float

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

###### `Boolean`

Represents a TOML boolean

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

###### `Datetime`

Represents a TOML datetime

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Datetime` |  |

###### `Array`

Represents a TOML array

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Array` |  |

###### `Table`

Represents a TOML table

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Table` |  |

##### Implementations

###### Methods

- ```rust
  pub fn try_from<T>(value: T) -> Result<Value, crate::ser::Error>
where
    T: ser::Serialize { /* ... */ }
  ```
  Convert a `T` into `toml::Value` which is an enum that can represent

- ```rust
  pub fn try_into<''de, T>(self: Self) -> Result<T, crate::de::Error>
where
    T: de::Deserialize<''de> { /* ... */ }
  ```
  Interpret a `toml::Value` as an instance of type `T`.

- ```rust
  pub fn get<I: Index>(self: &Self, index: I) -> Option<&Value> { /* ... */ }
  ```
  Index into a TOML array or map. A string index can be used to access a

- ```rust
  pub fn get_mut<I: Index>(self: &mut Self, index: I) -> Option<&mut Value> { /* ... */ }
  ```
  Mutably index into a TOML array or map. A string index can be used to

- ```rust
  pub fn as_integer(self: &Self) -> Option<i64> { /* ... */ }
  ```
  Extracts the integer value if it is an integer.

- ```rust
  pub fn is_integer(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is an integer.

- ```rust
  pub fn as_float(self: &Self) -> Option<f64> { /* ... */ }
  ```
  Extracts the float value if it is a float.

- ```rust
  pub fn is_float(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is a float.

- ```rust
  pub fn as_bool(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Extracts the boolean value if it is a boolean.

- ```rust
  pub fn is_bool(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is a boolean.

- ```rust
  pub fn as_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Extracts the string of this value if it is a string.

- ```rust
  pub fn is_str(self: &Self) -> bool { /* ... */ }
  ```
  Tests if this value is a string.

- ```rust
  pub fn as_datetime(self: &Self) -> Option<&Datetime> { /* ... */ }
  ```
  Extracts the datetime value if it is a datetime.

- ```rust
  pub fn is_datetime(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is a datetime.

- ```rust
  pub fn as_array(self: &Self) -> Option<&Vec<Value>> { /* ... */ }
  ```
  Extracts the array value if it is an array.

- ```rust
  pub fn as_array_mut(self: &mut Self) -> Option<&mut Vec<Value>> { /* ... */ }
  ```
  Extracts the array value if it is an array.

- ```rust
  pub fn is_array(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is an array.

- ```rust
  pub fn as_table(self: &Self) -> Option<&Table> { /* ... */ }
  ```
  Extracts the table value if it is a table.

- ```rust
  pub fn as_table_mut(self: &mut Self) -> Option<&mut Table> { /* ... */ }
  ```
  Extracts the table value if it is a table.

- ```rust
  pub fn is_table(self: &Self) -> bool { /* ... */ }
  ```
  Tests whether this value is a table.

- ```rust
  pub fn same_type(self: &Self, other: &Value) -> bool { /* ... */ }
  ```
  Tests whether this and another value have the same type.

- ```rust
  pub fn type_str(self: &Self) -> &''static str { /* ... */ }
  ```
  Returns a human-readable representation of the type of this value.

###### Trait Implementations

- **UnwindSafe**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: I) -> &mut Value { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: &''a str) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: Vec<V>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<S, V>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<S, V>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: String) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: i64) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: i32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: i8) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: u8) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: u32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: f64) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: f32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: bool) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: Datetime) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(val: Table) -> Value { /* ... */ }
    ```

- **Unpin**
- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, crate::de::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, _name: &''static str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, crate::de::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, crate::de::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value, crate::de::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>
where
    D: de::Deserializer<''de> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Index**
  - ```rust
    fn index(self: &Self, index: I) -> &Value { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **DeserializeOwned**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Value, <Self as >::Err> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

### Traits

#### Trait `Index`

Types that can be used to index a `toml::Value`

Currently this is implemented for `usize` to index arrays and `str` to index
tables.

This trait is sealed and not intended for implementation outside of the
`toml` crate.

```rust
pub trait Index: Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `usize`
- `str`
- `String`
- `&T` with <T>

### Re-exports

#### Re-export `Date`

```rust
pub use toml_datetime::Date;
```

#### Re-export `Datetime`

```rust
pub use toml_datetime::Datetime;
```

#### Re-export `DatetimeParseError`

```rust
pub use toml_datetime::DatetimeParseError;
```

#### Re-export `Offset`

```rust
pub use toml_datetime::Offset;
```

#### Re-export `Time`

```rust
pub use toml_datetime::Time;
```

#### Re-export `Table`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::Table;
```

## Module `de`

Deserializing TOML into Rust structures.

This module contains all the Serde support for deserializing TOML documents
into Rust structures. Note that some top-level functions here are also
provided at the top of the crate.

```rust
pub mod de { /* ... */ }
```

### Types

#### Struct `Error`

Errors that can occur when deserializing a type.

```rust
pub struct Error {
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
  pub fn message(self: &Self) -> &str { /* ... */ }
  ```
  What went wrong

- ```rust
  pub fn span(self: &Self) -> Option<std::ops::Range<usize>> { /* ... */ }
  ```
  The start/end index into the original document where the error occurred

###### Trait Implementations

- **Send**
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

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn custom<T>(msg: T) -> Self
where
    T: std::fmt::Display { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Deserializer`

**Attributes:**

- `#[<cfg>(feature = "parse")]`

Deserialization TOML document

To deserializes TOML values, instead of documents, see [`ValueDeserializer`].

```rust
pub struct Deserializer<''a> {
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
  pub fn new(input: &''a str) -> Self { /* ... */ }
  ```
  Deserialization implementation for TOML.

###### Trait Implementations

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `ValueDeserializer`

**Attributes:**

- `#[<cfg>(feature = "parse")]`

Deserialization TOML [value][crate::Value]

# Example

```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    title: String,
    owner: Owner,
}

#[derive(Deserialize)]
struct Owner {
    name: String,
}

let config = Config::deserialize(toml::de::ValueDeserializer::new(
    r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#
)).unwrap();

assert_eq!(config.title, "TOML Example");
assert_eq!(config.owner.name, "Lisa");
```

```rust
pub struct ValueDeserializer<''a> {
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
  pub fn new(input: &''a str) -> Self { /* ... */ }
  ```
  Deserialization implementation for TOML.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **RefUnwindSafe**
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

### Functions

#### Function `from_str`

**Attributes:**

- `#[<cfg>(feature = "parse")]`

Deserializes a string into a type.

This function will attempt to interpret `s` as a TOML document and
deserialize `T` from the document.

To deserializes TOML values, instead of documents, see [`ValueDeserializer`].

# Examples

```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    title: String,
    owner: Owner,
}

#[derive(Deserialize)]
struct Owner {
    name: String,
}

let config: Config = toml::from_str(r#"
    title = 'TOML Example'

    [owner]
    name = 'Lisa'
"#).unwrap();

assert_eq!(config.title, "TOML Example");
assert_eq!(config.owner.name, "Lisa");
```

```rust
pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned { /* ... */ }
```

## Module `ser`

Serializing Rust structures into TOML.

This module contains all the Serde support for serializing Rust structures
into TOML documents (as strings). Note that some top-level functions here
are also provided at the top of the crate.

```rust
pub mod ser { /* ... */ }
```

### Types

#### Struct `Error`

Errors that can occur when serializing a type.

```rust
pub struct Error {
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
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Error**
  - ```rust
    fn custom<T>(msg: T) -> Self
where
    T: std::fmt::Display { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

#### Struct `Serializer`

**Attributes:**

- `#[non_exhaustive]`
- `#[<cfg>(feature = "display")]`

Serialization for TOML documents.

This structure implements serialization support for TOML to serialize an
arbitrary type to TOML. Note that the TOML format does not support all
datatypes in Rust, such as enums, tuples, and tuple structs. These types
will generate an error when serialized.

Currently a serializer always writes its output to an in-memory `String`,
which is passed in when creating the serializer itself.

To serialize TOML values, instead of documents, see [`ValueSerializer`].

```rust
pub struct Serializer<''d> {
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
  pub fn new(dst: &''d mut String) -> Self { /* ... */ }
  ```
  Creates a new serializer which will emit TOML into the buffer provided.

- ```rust
  pub fn pretty(dst: &''d mut String) -> Self { /* ... */ }
  ```
  Apply a default "pretty" policy to the document

###### Trait Implementations

- **Unpin**
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
- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Serializer**
  - ```rust
    fn serialize_bool(self: Self, v: bool) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i8(self: Self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i16(self: Self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i32(self: Self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i64(self: Self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u8(self: Self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u16(self: Self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u32(self: Self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u64(self: Self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_f32(self: Self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_f64(self: Self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_char(self: Self, v: char) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_str(self: Self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_bytes(self: Self, v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_none(self: Self) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_some<T>(self: Self, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_unit(self: Self) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_struct(self: Self, name: &''static str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_variant(self: Self, name: &''static str, variant_index: u32, variant: &''static str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_struct<T>(self: Self, name: &''static str, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_variant<T>(self: Self, name: &''static str, variant_index: u32, variant: &''static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_struct(self: Self, _name: &''static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_variant(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct(self: Self, _name: &''static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct_variant(self: Self, name: &''static str, _variant_index: u32, _variant: &''static str, _len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
#### Struct `ValueSerializer`

**Attributes:**

- `#[non_exhaustive]`
- `#[<cfg>(feature = "display")]`

Serialization for TOML [values][crate::Value].

This structure implements serialization support for TOML to serialize an
arbitrary type to TOML. Note that the TOML format does not support all
datatypes in Rust, such as enums, tuples, and tuple structs. These types
will generate an error when serialized.

Currently a serializer always writes its output to an in-memory `String`,
which is passed in when creating the serializer itself.

# Examples

```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

let config = Config {
    database: Database {
        ip: "192.168.1.1".to_string(),
        port: vec![8001, 8002, 8003],
        connection_max: 5000,
        enabled: false,
    },
};

let mut value = String::new();
serde::Serialize::serialize(
    &config,
    toml::ser::ValueSerializer::new(&mut value)
).unwrap();
println!("{}", value)
```

```rust
pub struct ValueSerializer<''d> {
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
  pub fn new(dst: &''d mut String) -> Self { /* ... */ }
  ```
  Creates a new serializer which will emit TOML into the buffer provided.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Serializer**
  - ```rust
    fn serialize_bool(self: Self, v: bool) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i8(self: Self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i16(self: Self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i32(self: Self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_i64(self: Self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u8(self: Self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u16(self: Self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u32(self: Self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_u64(self: Self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_f32(self: Self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_f64(self: Self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_char(self: Self, v: char) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_str(self: Self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_bytes(self: Self, v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_none(self: Self) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_some<T>(self: Self, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_unit(self: Self) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_struct(self: Self, name: &''static str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_variant(self: Self, name: &''static str, variant_index: u32, variant: &''static str) -> Result<<Self as >::Ok, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_struct<T>(self: Self, name: &''static str, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_variant<T>(self: Self, name: &''static str, variant_index: u32, variant: &''static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
    ```

  - ```rust
    fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_struct(self: Self, _name: &''static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_variant(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct(self: Self, _name: &''static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct_variant(self: Self, name: &''static str, _variant_index: u32, _variant: &''static str, _len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error> { /* ... */ }
    ```

### Functions

#### Function `to_string`

**Attributes:**

- `#[<cfg>(feature = "display")]`

Serialize the given data structure as a String of TOML.

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, if `T` contains a map with non-string keys, or if `T` attempts to
serialize an unsupported datatype such as an enum, tuple, or tuple struct.

To serialize TOML values, instead of documents, see [`ValueSerializer`].

# Examples

```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

let config = Config {
    database: Database {
        ip: "192.168.1.1".to_string(),
        port: vec![8001, 8002, 8003],
        connection_max: 5000,
        enabled: false,
    },
};

let toml = toml::to_string(&config).unwrap();
println!("{}", toml)
```

```rust
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

#### Function `to_string_pretty`

**Attributes:**

- `#[<cfg>(feature = "display")]`

Serialize the given data structure as a "pretty" String of TOML.

This is identical to `to_string` except the output string has a more
"pretty" output. See `Serializer::pretty` for more details.

To serialize TOML values, instead of documents, see [`ValueSerializer`].

For greater customization, instead serialize to a
[`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html).

```rust
pub fn to_string_pretty<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

## Macros

### Macro `toml`

**Attributes:**

- `#[macro_export]`

Construct a [`Table`] from TOML syntax.

```rust
let cargo_toml = toml::toml! {
    [package]
    name = "toml"

    [dependencies]
    serde = "1.0"

    [dev-dependencies]
    serde_derive = "1.0"
    serde_json = "1.0"
};

println!("{:#?}", cargo_toml);
```

```rust
pub macro_rules! toml {
    /* macro_rules! toml {
    ($($toml:tt)+) => { ... };
} */
}
```

## Re-exports

### Re-export `from_str`

**Attributes:**

- `#[<cfg>(feature = "parse")]`
- `#[doc(inline)]`

```rust
pub use crate::de::from_str;
```

### Re-export `Deserializer`

**Attributes:**

- `#[<cfg>(feature = "parse")]`
- `#[doc(inline)]`

```rust
pub use crate::de::Deserializer;
```

### Re-export `to_string`

**Attributes:**

- `#[<cfg>(feature = "display")]`
- `#[doc(inline)]`

```rust
pub use crate::ser::to_string;
```

### Re-export `to_string_pretty`

**Attributes:**

- `#[<cfg>(feature = "display")]`
- `#[doc(inline)]`

```rust
pub use crate::ser::to_string_pretty;
```

### Re-export `Serializer`

**Attributes:**

- `#[<cfg>(feature = "display")]`
- `#[doc(inline)]`

```rust
pub use crate::ser::Serializer;
```

### Re-export `Value`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::Value;
```

### Re-export `Spanned`

```rust
pub use serde_spanned::Spanned;
```

### Re-export `Table`

```rust
pub use table::Table;
```

