# Crate Documentation

**Version:** 0.6.4

**Format Version:** 43

# Module `rand_core`

Random number generation traits

This crate is mainly of interest to crates publishing implementations of
[`RngCore`]. Other users are encouraged to use the [`rand`] crate instead
which re-exports the main traits and error types.

[`RngCore`] is the core trait implemented by algorithmic pseudo-random number
generators and external random-number sources.

[`SeedableRng`] is an extension trait for construction from fixed seeds and
other random number generators.

[`Error`] is provided for error-handling. It is safe to use in `no_std`
environments.

The [`impls`] and [`le`] sub-modules include a few small functions to assist
implementation of [`RngCore`].

[`rand`]: https://docs.rs/rand

## Modules

## Module `block`

The `BlockRngCore` trait and implementation helpers

The [`BlockRngCore`] trait exists to assist in the implementation of RNGs
which generate a block of data in a cache instead of returning generated
values directly.

Usage of this trait is optional, but provides two advantages:
implementations only need to concern themselves with generation of the
block, not the various [`RngCore`] methods (especially [`fill_bytes`], where
the optimal implementations are not trivial), and this allows
`ReseedingRng` (see [`rand`](https://docs.rs/rand) crate) perform periodic
reseeding with very low overhead.

# Example

```no_run
use rand_core::{RngCore, SeedableRng};
use rand_core::block::{BlockRngCore, BlockRng};

struct MyRngCore;

impl BlockRngCore for MyRngCore {
    type Item = u32;
    type Results = [u32; 16];

    fn generate(&mut self, results: &mut Self::Results) {
        unimplemented!()
    }
}

impl SeedableRng for MyRngCore {
    type Seed = [u8; 32];
    fn from_seed(seed: Self::Seed) -> Self {
        unimplemented!()
    }
}

// optionally, also implement CryptoRng for MyRngCore

// Final RNG.
let mut rng = BlockRng::<MyRngCore>::seed_from_u64(0);
println!("First value: {}", rng.next_u32());
```

[`BlockRngCore`]: crate::block::BlockRngCore
[`fill_bytes`]: RngCore::fill_bytes

```rust
pub mod block { /* ... */ }
```

### Types

#### Struct `BlockRng`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`
- `#[<cfg_attr>(feature = "serde1",
serde(bound =
"for<'x> R: Serialize + Deserialize<'x> + Sized, for<'x> R::Results: Serialize + Deserialize<'x>"))]`

A wrapper type implementing [`RngCore`] for some type implementing
[`BlockRngCore`] with `u32` array buffer; i.e. this can be used to implement
a full RNG from just a `generate` function.

The `core` field may be accessed directly but the results buffer may not.
PRNG implementations can simply use a type alias
(`pub type MyRng = BlockRng<MyRngCore>;`) but might prefer to use a
wrapper type (`pub struct MyRng(BlockRng<MyRngCore>);`); the latter must
re-implement `RngCore` but hides the implementation details and allows
extra functionality to be defined on the RNG
(e.g. `impl MyRng { fn set_stream(...){...} }`).

`BlockRng` has heavily optimized implementations of the [`RngCore`] methods
reading values from the results buffer, as well as
calling [`BlockRngCore::generate`] directly on the output array when
[`fill_bytes`] / [`try_fill_bytes`] is called on a large array. These methods
also handle the bookkeeping of when to generate a new batch of values.

No whole generated `u32` values are thrown away and all values are consumed
in-order. [`next_u32`] simply takes the next available `u32` value.
[`next_u64`] is implemented by combining two `u32` values, least
significant first. [`fill_bytes`] and [`try_fill_bytes`] consume a whole
number of `u32` values, converting each `u32` to a byte slice in
little-endian order. If the requested byte length is not a multiple of 4,
some bytes will be discarded.

See also [`BlockRng64`] which uses `u64` array buffers. Currently there is
no direct support for other buffer types.

For easy initialization `BlockRng` also implements [`SeedableRng`].

[`next_u32`]: RngCore::next_u32
[`next_u64`]: RngCore::next_u64
[`fill_bytes`]: RngCore::fill_bytes
[`try_fill_bytes`]: RngCore::try_fill_bytes

```rust
pub struct BlockRng<R: BlockRngCore + ?Sized> {
    pub core: R,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `core` | `R` | The *core* part of the RNG, implementing the `generate` function. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(core: R) -> BlockRng<R> { /* ... */ }
  ```
  Create a new `BlockRng` from an existing RNG implementing

- ```rust
  pub fn index(self: &Self) -> usize { /* ... */ }
  ```
  Get the index into the result buffer.

- ```rust
  pub fn reset(self: &mut Self) { /* ... */ }
  ```
  Reset the number of available results.

- ```rust
  pub fn generate_and_set(self: &mut Self, index: usize) { /* ... */ }
  ```
  Generate a new set of results immediately, setting the index to the

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RngCore**
  - ```rust
    fn next_u32(self: &mut Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn next_u64(self: &mut Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn fill_bytes(self: &mut Self, dest: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn try_fill_bytes(self: &mut Self, dest: &mut [u8]) -> Result<(), Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BlockRng<R> { /* ... */ }
    ```

- **SeedableRng**
  - ```rust
    fn from_seed(seed: <Self as >::Seed) -> Self { /* ... */ }
    ```

  - ```rust
    fn seed_from_u64(seed: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_rng<S: RngCore>(rng: S) -> Result<Self, Error> { /* ... */ }
    ```

- **CryptoRngCore**
  - ```rust
    fn as_rngcore(self: &mut Self) -> &mut dyn RngCore { /* ... */ }
    ```

- **CryptoRng**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `BlockRng64`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

A wrapper type implementing [`RngCore`] for some type implementing
[`BlockRngCore`] with `u64` array buffer; i.e. this can be used to implement
a full RNG from just a `generate` function.

This is similar to [`BlockRng`], but specialized for algorithms that operate
on `u64` values.

No whole generated `u64` values are thrown away and all values are consumed
in-order. [`next_u64`] simply takes the next available `u64` value.
[`next_u32`] is however a bit special: half of a `u64` is consumed, leaving
the other half in the buffer. If the next function called is [`next_u32`]
then the other half is then consumed, however both [`next_u64`] and
[`fill_bytes`] discard the rest of any half-consumed `u64`s when called.

[`fill_bytes`] and [`try_fill_bytes`] consume a whole number of `u64`
values. If the requested length is not a multiple of 8, some bytes will be
discarded.

[`next_u32`]: RngCore::next_u32
[`next_u64`]: RngCore::next_u64
[`fill_bytes`]: RngCore::fill_bytes
[`try_fill_bytes`]: RngCore::try_fill_bytes

```rust
pub struct BlockRng64<R: BlockRngCore + ?Sized> {
    pub core: R,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `core` | `R` | The *core* part of the RNG, implementing the `generate` function. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(core: R) -> BlockRng64<R> { /* ... */ }
  ```
  Create a new `BlockRng` from an existing RNG implementing

- ```rust
  pub fn index(self: &Self) -> usize { /* ... */ }
  ```
  Get the index into the result buffer.

- ```rust
  pub fn reset(self: &mut Self) { /* ... */ }
  ```
  Reset the number of available results.

- ```rust
  pub fn generate_and_set(self: &mut Self, index: usize) { /* ... */ }
  ```
  Generate a new set of results immediately, setting the index to the

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **SeedableRng**
  - ```rust
    fn from_seed(seed: <Self as >::Seed) -> Self { /* ... */ }
    ```

  - ```rust
    fn seed_from_u64(seed: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_rng<S: RngCore>(rng: S) -> Result<Self, Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BlockRng64<R> { /* ... */ }
    ```

- **RngCore**
  - ```rust
    fn next_u32(self: &mut Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn next_u64(self: &mut Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn fill_bytes(self: &mut Self, dest: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn try_fill_bytes(self: &mut Self, dest: &mut [u8]) -> Result<(), Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `BlockRngCore`

A trait for RNGs which do not generate random numbers individually, but in
blocks (typically `[u32; N]`). This technique is commonly used by
cryptographic RNGs to improve performance.

See the [module][crate::block] documentation for details.

```rust
pub trait BlockRngCore {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Item`: Results element type, e.g. `u32`.
- `Results`: Results type. This is the 'block' an RNG implementing `BlockRngCore`

###### Required Methods

- `generate`: Generate a new block of results.

## Module `impls`

Helper functions for implementing `RngCore` functions.

For cross-platform reproducibility, these functions all use Little Endian:
least-significant part first. For example, `next_u64_via_u32` takes `u32`
values `x, y`, then outputs `(y << 32) | x`. To implement `next_u32`
from `next_u64` in little-endian order, one should use `next_u64() as u32`.

Byte-swapping (like the std `to_le` functions) is only needed to convert
to/from byte sequences, and since its purpose is reproducibility,
non-reproducible sources (e.g. `OsRng`) need not bother with it.

```rust
pub mod impls { /* ... */ }
```

### Functions

#### Function `next_u64_via_u32`

Implement `next_u64` via `next_u32`, little-endian order.

```rust
pub fn next_u64_via_u32<R: RngCore + ?Sized>(rng: &mut R) -> u64 { /* ... */ }
```

#### Function `fill_bytes_via_next`

Implement `fill_bytes` via `next_u64` and `next_u32`, little-endian order.

The fastest way to fill a slice is usually to work as long as possible with
integers. That is why this method mostly uses `next_u64`, and only when
there are 4 or less bytes remaining at the end of the slice it uses
`next_u32` once.

```rust
pub fn fill_bytes_via_next<R: RngCore + ?Sized>(rng: &mut R, dest: &mut [u8]) { /* ... */ }
```

#### Function `fill_via_u32_chunks`

Implement `fill_bytes` by reading chunks from the output buffer of a block
based RNG.

The return values are `(consumed_u32, filled_u8)`.

`filled_u8` is the number of filled bytes in `dest`, which may be less than
the length of `dest`.
`consumed_u32` is the number of words consumed from `src`, which is the same
as `filled_u8 / 4` rounded up.

# Example
(from `IsaacRng`)

```ignore
fn fill_bytes(&mut self, dest: &mut [u8]) {
    let mut read_len = 0;
    while read_len < dest.len() {
        if self.index >= self.rsl.len() {
            self.isaac();
        }

        let (consumed_u32, filled_u8) =
            impls::fill_via_u32_chunks(&mut self.rsl[self.index..],
                                       &mut dest[read_len..]);

        self.index += consumed_u32;
        read_len += filled_u8;
    }
}
```

```rust
pub fn fill_via_u32_chunks(src: &[u32], dest: &mut [u8]) -> (usize, usize) { /* ... */ }
```

#### Function `fill_via_u64_chunks`

Implement `fill_bytes` by reading chunks from the output buffer of a block
based RNG.

The return values are `(consumed_u64, filled_u8)`.
`filled_u8` is the number of filled bytes in `dest`, which may be less than
the length of `dest`.
`consumed_u64` is the number of words consumed from `src`, which is the same
as `filled_u8 / 8` rounded up.

See `fill_via_u32_chunks` for an example.

```rust
pub fn fill_via_u64_chunks(src: &[u64], dest: &mut [u8]) -> (usize, usize) { /* ... */ }
```

#### Function `next_u32_via_fill`

Implement `next_u32` via `fill_bytes`, little-endian order.

```rust
pub fn next_u32_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u32 { /* ... */ }
```

#### Function `next_u64_via_fill`

Implement `next_u64` via `fill_bytes`, little-endian order.

```rust
pub fn next_u64_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u64 { /* ... */ }
```

## Module `le`

Little-Endian utilities

Little-Endian order has been chosen for internal usage; this makes some
useful functions available.

```rust
pub mod le { /* ... */ }
```

### Functions

#### Function `read_u32_into`

**Attributes:**

- `#[inline]`

Reads unsigned 32 bit integers from `src` into `dst`.

```rust
pub fn read_u32_into(src: &[u8], dst: &mut [u32]) { /* ... */ }
```

#### Function `read_u64_into`

**Attributes:**

- `#[inline]`

Reads unsigned 64 bit integers from `src` into `dst`.

```rust
pub fn read_u64_into(src: &[u8], dst: &mut [u64]) { /* ... */ }
```

## Traits

### Trait `RngCore`

The core of a random number generator.

This trait encapsulates the low-level functionality common to all
generators, and is the "back end", to be implemented by generators.
End users should normally use the `Rng` trait from the [`rand`] crate,
which is automatically implemented for every type implementing `RngCore`.

Three different methods for generating random data are provided since the
optimal implementation of each is dependent on the type of generator. There
is no required relationship between the output of each; e.g. many
implementations of [`fill_bytes`] consume a whole number of `u32` or `u64`
values and drop any remaining unused bytes. The same can happen with the
[`next_u32`] and [`next_u64`] methods, implementations may discard some
random bits for efficiency.

The [`try_fill_bytes`] method is a variant of [`fill_bytes`] allowing error
handling; it is not deemed sufficiently useful to add equivalents for
[`next_u32`] or [`next_u64`] since the latter methods are almost always used
with algorithmic generators (PRNGs), which are normally infallible.

Implementers should produce bits uniformly. Pathological RNGs (e.g. always
returning the same value, or never setting certain bits) can break rejection
sampling used by random distributions, and also break other RNGs when
seeding them via [`SeedableRng::from_rng`].

Algorithmic generators implementing [`SeedableRng`] should normally have
*portable, reproducible* output, i.e. fix Endianness when converting values
to avoid platform differences, and avoid making any changes which affect
output (except by communicating that the release has breaking changes).

Typically an RNG will implement only one of the methods available
in this trait directly, then use the helper functions from the
[`impls`] module to implement the other methods.

It is recommended that implementations also implement:

- `Debug` with a custom implementation which *does not* print any internal
  state (at least, [`CryptoRng`]s should not risk leaking state through
  `Debug`).
- `Serialize` and `Deserialize` (from Serde), preferably making Serde
  support optional at the crate level in PRNG libs.
- `Clone`, if possible.
- *never* implement `Copy` (accidental copies may cause repeated values).
- *do not* implement `Default` for pseudorandom generators, but instead
  implement [`SeedableRng`], to guide users towards proper seeding.
  External / hardware RNGs can choose to implement `Default`.
- `Eq` and `PartialEq` could be implemented, but are probably not useful.

# Example

A simple example, obviously not generating very *random* output:

```
#![allow(dead_code)]
use rand_core::{RngCore, Error, impls};

struct CountingRng(u64);

impl RngCore for CountingRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
```

[`rand`]: https://docs.rs/rand
[`try_fill_bytes`]: RngCore::try_fill_bytes
[`fill_bytes`]: RngCore::fill_bytes
[`next_u32`]: RngCore::next_u32
[`next_u64`]: RngCore::next_u64

```rust
pub trait RngCore {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `next_u32`: Return the next random `u32`.
- `next_u64`: Return the next random `u64`.
- `fill_bytes`: Fill `dest` with random data.
- `try_fill_bytes`: Fill `dest` entirely with random data.

#### Implementations

This trait is implemented for the following types:

- `BlockRng<R>` with <R: BlockRngCore<Item = u32>>
- `BlockRng64<R>` with <R: BlockRngCore<Item = u64>>
- `OsRng`
- `&''a mut R` with <''a, R: RngCore + ?Sized>
- `alloc::boxed::Box<R>` with <R: RngCore + ?Sized>

### Trait `CryptoRng`

A marker trait used to indicate that an [`RngCore`] or [`BlockRngCore`]
implementation is supposed to be cryptographically secure.

*Cryptographically secure generators*, also known as *CSPRNGs*, should
satisfy an additional properties over other generators: given the first
*k* bits of an algorithm's output
sequence, it should not be possible using polynomial-time algorithms to
predict the next bit with probability significantly greater than 50%.

Some generators may satisfy an additional property, however this is not
required by this trait: if the CSPRNG's state is revealed, it should not be
computationally-feasible to reconstruct output prior to this. Some other
generators allow backwards-computation and are considered *reversible*.

Note that this trait is provided for guidance only and cannot guarantee
suitability for cryptographic applications. In general it should only be
implemented for well-reviewed code implementing well-regarded algorithms.

Note also that use of a `CryptoRng` does not protect against other
weaknesses such as seeding from a weak entropy source or leaking state.

[`BlockRngCore`]: block::BlockRngCore

```rust
pub trait CryptoRng {
    /* Associated items */
}
```

#### Implementations

This trait is implemented for the following types:

- `BlockRng<R>` with <R: BlockRngCore + CryptoRng>
- `OsRng`
- `&''a mut R` with <''a, R: CryptoRng + ?Sized>
- `alloc::boxed::Box<R>` with <R: CryptoRng + ?Sized>

### Trait `CryptoRngCore`

An extension trait that is automatically implemented for any type
implementing [`RngCore`] and [`CryptoRng`].

It may be used as a trait object, and supports upcasting to [`RngCore`] via
the [`CryptoRngCore::as_rngcore`] method.

# Example

```
use rand_core::CryptoRngCore;

#[allow(unused)]
fn make_token(rng: &mut dyn CryptoRngCore) -> [u8; 32] {
    let mut buf = [0u8; 32];
    rng.fill_bytes(&mut buf);
    buf
}
```

```rust
pub trait CryptoRngCore: CryptoRng + RngCore {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `as_rngcore`: Upcast to an [`RngCore`] trait object.

#### Implementations

This trait is implemented for the following types:

- `T` with <T: CryptoRng + RngCore>

### Trait `SeedableRng`

A random number generator that can be explicitly seeded.

This trait encapsulates the low-level functionality common to all
pseudo-random number generators (PRNGs, or algorithmic generators).

[`rand`]: https://docs.rs/rand

```rust
pub trait SeedableRng: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Seed`: Seed type, which is restricted to types mutably-dereferenceable as `u8`

##### Required Methods

- `from_seed`: Create a new PRNG using the given seed.

#### Provided Methods

- ```rust
  fn seed_from_u64(state: u64) -> Self { /* ... */ }
  ```
  Create a new PRNG using a `u64` seed.

- ```rust
  fn from_rng<R: RngCore>(rng: R) -> Result<Self, Error> { /* ... */ }
  ```
  Create a new PRNG seeded from another `Rng`.

- ```rust
  fn from_entropy() -> Self { /* ... */ }
  ```
  Creates a new instance of the RNG seeded via [`getrandom`].

#### Implementations

This trait is implemented for the following types:

- `BlockRng<R>` with <R: BlockRngCore + SeedableRng>
- `BlockRng64<R>` with <R: BlockRngCore + SeedableRng>

## Re-exports

### Re-export `Error`

```rust
pub use error::Error;
```

### Re-export `OsRng`

**Attributes:**

- `#[<cfg>(feature = "getrandom")]`

```rust
pub use os::OsRng;
```

