# Crate Documentation

**Version:** 1.0.7

**Format Version:** 43

# Module `fnv`

An implementation of the [Fowler–Noll–Vo hash function][chongo].

## About

The FNV hash function is a custom `Hasher` implementation that is more
efficient for smaller hash keys.

[The Rust FAQ states that][faq] while the default `Hasher` implementation,
SipHash, is good in many cases, it is notably slower than other algorithms
with short keys, such as when you have a map of integers to other values.
In cases like these, [FNV is demonstrably faster][graphs].

Its disadvantages are that it performs badly on larger inputs, and
provides no protection against collision attacks, where a malicious user
can craft specific keys designed to slow a hasher down. Thus, it is
important to profile your program to ensure that you are using small hash
keys, and be certain that your program could not be exposed to malicious
inputs (including being a networked server).

The Rust compiler itself uses FNV, as it is not worried about
denial-of-service attacks, and can assume that its inputs are going to be
small—a perfect use case for FNV.


## Using FNV in a `HashMap`

The `FnvHashMap` type alias is the easiest way to use the standard library’s
`HashMap` with FNV.

```rust
use fnv::FnvHashMap;

let mut map = FnvHashMap::default();
map.insert(1, "one");
map.insert(2, "two");

map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
map.insert(1, "one");
map.insert(2, "two");
```

Note, the standard library’s `HashMap::new` and `HashMap::with_capacity`
are only implemented for the `RandomState` hasher, so using `Default` to
get the hasher is the next best option.

## Using FNV in a `HashSet`

Similarly, `FnvHashSet` is a type alias for the standard library’s `HashSet`
with FNV.

```rust
use fnv::FnvHashSet;

let mut set = FnvHashSet::default();
set.insert(1);
set.insert(2);

set = FnvHashSet::with_capacity_and_hasher(10, Default::default());
set.insert(1);
set.insert(2);
```

[chongo]: http://www.isthe.com/chongo/tech/comp/fnv/index.html
[faq]: https://www.rust-lang.org/en-US/faq.html#why-are-rusts-hashmaps-slow
[graphs]: https://cglab.ca/~abeinges/blah/hash-rs/

## Types

### Struct `FnvHasher`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

An implementation of the Fowler–Noll–Vo hash function.

See the [crate documentation](index.html) for more details.

```rust
pub struct FnvHasher(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn with_key(key: u64) -> FnvHasher { /* ... */ }
  ```
  Create an FNV hasher starting with a state corresponding

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

- **Default**
  - ```rust
    fn default() -> FnvHasher { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **UnwindSafe**
- **Hasher**
  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn write(self: &mut Self, bytes: &[u8]) { /* ... */ }
    ```

### Type Alias `FnvBuildHasher`

A builder for default FNV hashers.

```rust
pub type FnvBuildHasher = std::hash::BuildHasherDefault<FnvHasher>;
```

### Type Alias `FnvHashMap`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A `HashMap` using a default FNV hasher.

```rust
pub type FnvHashMap<K, V> = std::collections::HashMap<K, V, FnvBuildHasher>;
```

### Type Alias `FnvHashSet`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A `HashSet` using a default FNV hasher.

```rust
pub type FnvHashSet<T> = std::collections::HashSet<T, FnvBuildHasher>;
```

