# Crate Documentation

**Version:** 0.1.5

**Format Version:** 43

# Module `foldhash`

This crate provides foldhash, a fast, non-cryptographic, minimally
DoS-resistant hashing algorithm designed for computational uses such as
hashmaps, bloom filters, count sketching, etc.

When should you **not** use foldhash:

- You are afraid of people studying your long-running program's behavior
  to reverse engineer its internal random state and using this knowledge to
  create many colliding inputs for computational complexity attacks.

- You expect foldhash to have a consistent output across versions or
  platforms, such as for persistent file formats or communication protocols.
   
- You are relying on foldhash's properties for any kind of security.
  Foldhash is **not appropriate for any cryptographic purpose**.

Foldhash has two variants, one optimized for speed which is ideal for data
structures such as hash maps and bloom filters, and one optimized for
statistical quality which is ideal for algorithms such as
[HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog) and
[MinHash](https://en.wikipedia.org/wiki/MinHash).

Foldhash can be used in a `#![no_std]` environment by disabling its default
`"std"` feature.

# Usage

The easiest way to use this crate with the standard library [`HashMap`] or
[`HashSet`] is to import them from `foldhash` instead, along with the
extension traits to make [`HashMap::new`] and [`HashMap::with_capacity`]
work out-of-the-box:

```rust
use foldhash::{HashMap, HashMapExt};

let mut hm = HashMap::new();
hm.insert(42, "hello");
```

You can also avoid the convenience types and do it manually by initializing
a [`RandomState`](fast::RandomState), for example if you are using a different hash map
implementation like [`hashbrown`](https://docs.rs/hashbrown/):

```rust
use hashbrown::HashMap;
use foldhash::fast::RandomState;

let mut hm = HashMap::with_hasher(RandomState::default());
hm.insert("foo", "bar");
```

The above methods are the recommended way to use foldhash, which will
automatically generate a randomly generated hasher instance for you. If you
absolutely must have determinism you can use [`FixedState`](fast::FixedState)
instead, but note that this makes you trivially vulnerable to HashDoS
attacks and might lead to quadratic runtime when moving data from one
hashmap/set into another:

```rust
use std::collections::HashSet;
use foldhash::fast::FixedState;

let mut hm = HashSet::with_hasher(FixedState::with_seed(42));
hm.insert([1, 10, 100]);
```

If you rely on statistical properties of the hash for the correctness of
your algorithm, such as in [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog),
it is suggested to use the [`RandomState`](quality::RandomState)
or [`FixedState`](quality::FixedState) from the [`quality`] module instead
of the [`fast`] module. The latter is optimized purely for speed in hash
tables and has known statistical imperfections.

Finally, you can also directly use the [`RandomState`](quality::RandomState)
or [`FixedState`](quality::FixedState) to manually hash items using the
[`BuildHasher`](std::hash::BuildHasher) trait:
```rust
use std::hash::BuildHasher;
use foldhash::quality::RandomState;

let random_state = RandomState::default();
let hash = random_state.hash_one("hello world");
```

## Seeding

Foldhash relies on a single 8-byte per-hasher seed which should be ideally
be different from each instance to instance, and also a larger
[`SharedSeed`] which may be shared by many different instances.

To reduce overhead, this [`SharedSeed`] is typically initialized once and
stored. To prevent each hashmap unnecessarily containing a reference to this
value there are three kinds of [`BuildHasher`](core::hash::BuildHasher)s
foldhash provides (both for [`fast`] and [`quality`]):

1. [`RandomState`](fast::RandomState), which always generates a
   random per-hasher seed and implicitly stores a reference to [`SharedSeed::global_random`].
2. [`FixedState`](fast::FixedState), which by default uses a fixed
   per-hasher seed and implicitly stores a reference to [`SharedSeed::global_fixed`].
3. [`SeedableRandomState`](fast::SeedableRandomState), which works like
   [`RandomState`](fast::RandomState) by default but can be seeded in any manner.
   This state must include an explicit reference to a [`SharedSeed`], and thus
   this struct is 16 bytes as opposed to just 8 bytes for the previous two.

## Modules

## Module `fast`

The foldhash implementation optimized for speed.

```rust
pub mod fast { /* ... */ }
```

### Types

#### Struct `FoldHasher`

A [`Hasher`] instance implementing foldhash, optimized for speed.

While you can create one directly with [`FoldHasher::with_seed`], you
most likely want to use [`RandomState`], [`SeedableRandomState`] or
[`FixedState`] to create [`FoldHasher`]s.

```rust
pub struct FoldHasher {
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
  pub fn with_seed(per_hasher_seed: u64, shared_seed: &SharedSeed) -> FoldHasher { /* ... */ }
  ```
  Initializes this [`FoldHasher`] with the given per-hasher seed and

###### Trait Implementations

- **Unpin**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `RandomState`

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.

```rust
pub struct RandomState {
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RandomState { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

#### Struct `SeedableRandomState`

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`] is.

```rust
pub struct SeedableRandomState {
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
  pub fn random() -> Self { /* ... */ }
  ```
  Generates a random [`SeedableRandomState`], similar to [`RandomState`].

- ```rust
  pub fn fixed() -> Self { /* ... */ }
  ```
  Generates a fixed [`SeedableRandomState`], similar to [`FixedState`].

- ```rust
  pub fn with_seed(per_hasher_seed: u64, shared_seed: &''static SharedSeed) -> Self { /* ... */ }
  ```
  Generates a [`SeedableRandomState`] with the given per-hasher seed

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SeedableRandomState { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Sync**
#### Struct `FixedState`

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

```rust
pub struct FixedState {
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
  pub const fn with_seed(per_hasher_seed: u64) -> Self { /* ... */ }
  ```
  Creates a [`FixedState`] with the given per-hasher-seed.

###### Trait Implementations

- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FixedState { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
## Module `quality`

The foldhash implementation optimized for quality.

```rust
pub mod quality { /* ... */ }
```

### Types

#### Struct `FoldHasher`

A [`Hasher`] instance implementing foldhash, optimized for quality.

While you can create one directly with [`FoldHasher::with_seed`], you
most likely want to use [`RandomState`], [`SeedableRandomState`] or
[`FixedState`] to create [`FoldHasher`]s.

```rust
pub struct FoldHasher {
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
  pub fn with_seed(per_hasher_seed: u64, shared_seed: &SharedSeed) -> FoldHasher { /* ... */ }
  ```
  Initializes this [`FoldHasher`] with the given per-hasher seed and

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RandomState`

A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that is randomly initialized.

```rust
pub struct RandomState {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> RandomState { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RandomState { /* ... */ }
    ```

#### Struct `SeedableRandomState`

A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`] is.

```rust
pub struct SeedableRandomState {
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
  pub fn random() -> Self { /* ... */ }
  ```
  Generates a random [`SeedableRandomState`], similar to [`RandomState`].

- ```rust
  pub fn fixed() -> Self { /* ... */ }
  ```
  Generates a fixed [`SeedableRandomState`], similar to [`FixedState`].

- ```rust
  pub fn with_seed(per_hasher_seed: u64, shared_seed: &''static SharedSeed) -> Self { /* ... */ }
  ```
  Generates a [`SeedableRandomState`] with the given per-hasher seed

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> SeedableRandomState { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SeedableRandomState { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
#### Struct `FixedState`

A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

```rust
pub struct FixedState {
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
  pub const fn with_seed(per_hasher_seed: u64) -> Self { /* ... */ }
  ```
  Creates a [`FixedState`] with the given per-hasher seed.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BuildHasher**
  - ```rust
    fn build_hasher(self: &Self) -> FoldHasher { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> FixedState { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FixedState { /* ... */ }
    ```

## Re-exports

### Re-export `SharedSeed`

```rust
pub use seed::SharedSeed;
```

