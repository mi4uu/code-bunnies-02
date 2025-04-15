# Crate Documentation

**Version:** 0.15.2

**Format Version:** 43

# Module `hashbrown`

This crate is a Rust port of Google's high-performance [SwissTable] hash
map, adapted to make it a drop-in replacement for Rust's standard `HashMap`
and `HashSet` types.

The original C++ version of [SwissTable] can be found [here], and this
[CppCon talk] gives an overview of how the algorithm works.

[SwissTable]: https://abseil.io/blog/20180927-swisstables
[here]: https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h
[CppCon talk]: https://www.youtube.com/watch?v=ncHmEUmJZf4

## Modules

## Module `hash_map`

A hash map implemented with quadratic probing and SIMD lookup.

```rust
pub mod hash_map { /* ... */ }
```

### Re-exports

#### Re-export `crate::map::*`

```rust
pub use crate::map::*;
```

## Module `hash_set`

A hash set implemented as a `HashMap` where the value is `()`.

```rust
pub mod hash_set { /* ... */ }
```

### Re-exports

#### Re-export `crate::set::*`

```rust
pub use crate::set::*;
```

## Module `hash_table`

A hash table implemented with quadratic probing and SIMD lookup.

```rust
pub mod hash_table { /* ... */ }
```

### Re-exports

#### Re-export `crate::table::*`

```rust
pub use crate::table::*;
```

## Types

### Type Alias `DefaultHashBuilder`

**Attributes:**

- `#[<cfg>(feature = "default-hasher")]`

Default hasher for [`HashMap`] and [`HashSet`].

```rust
pub type DefaultHashBuilder = foldhash::fast::RandomState;
```

### Enum `TryReserveError`

The error type for `try_reserve` methods.

```rust
pub enum TryReserveError {
    CapacityOverflow,
    AllocError {
        layout: alloc::alloc::Layout,
    },
}
```

#### Variants

##### `CapacityOverflow`

Error due to the computed capacity exceeding the collection's maximum
(usually `isize::MAX` bytes).

##### `AllocError`

The memory allocator returned an error

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `layout` | `alloc::alloc::Layout` | The layout of the allocation request that failed. |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryReserveError { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryReserveError) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Sync**
## Traits

### Trait `Equivalent`

**Attributes:**

- `#[<cfg>(not(feature = "equivalent"))]`

Key equivalence trait.

This trait defines the function used to compare the input value with the
map keys (or set values) during a lookup operation such as [`HashMap::get`]
or [`HashSet::contains`].
It is provided with a blanket implementation based on the
[`Borrow`](core::borrow::Borrow) trait.

# Correctness

Equivalent values must hash to the same value.

```rust
pub trait Equivalent<K: ?Sized> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `equivalent`: Checks if this value is equivalent to the given key.

#### Implementations

This trait is implemented for the following types:

- `Q` with <Q, K>

## Re-exports

### Re-export `HashMap`

```rust
pub use crate::map::HashMap;
```

### Re-export `HashSet`

```rust
pub use crate::set::HashSet;
```

### Re-export `HashTable`

```rust
pub use crate::table::HashTable;
```

