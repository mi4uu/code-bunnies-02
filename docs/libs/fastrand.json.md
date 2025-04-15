# Crate Documentation

**Version:** 2.3.0

**Format Version:** 43

# Module `fastrand`

A simple and fast random number generator.

The implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
generator but **not** cryptographically secure.

# Examples

Flip a coin:

```
if fastrand::bool() {
    println!("heads");
} else {
    println!("tails");
}
```

Generate a random `i32`:

```
let num = fastrand::i32(..);
```

Choose a random element in an array:

```
let v = vec![1, 2, 3, 4, 5];
let i = fastrand::usize(..v.len());
let elem = v[i];
```

Sample values from an array with `O(n)` complexity (`n` is the length of array):

```
fastrand::choose_multiple([1, 4, 5], 2);
fastrand::choose_multiple(0..20, 12);
```


Shuffle an array:

```
let mut v = vec![1, 2, 3, 4, 5];
fastrand::shuffle(&mut v);
```

Generate a random [`Vec`] or [`String`]:

```
use std::iter::repeat_with;

let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
```

To get reproducible results on every run, initialize the generator with a seed:

```
// Pick an arbitrary number as seed.
fastrand::seed(7);

// Now this prints the same number on every run:
println!("{}", fastrand::u32(..));
```

To be more efficient, create a new [`Rng`] instance instead of using the thread-local
generator:

```
use std::iter::repeat_with;

let mut rng = fastrand::Rng::new();
let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
```

This crate aims to expose a core set of useful randomness primitives. For more niche algorithms,
consider using the [`fastrand-contrib`] crate alongside this one.

# Features

- `std` (enabled by default): Enables the `std` library. This is required for the global
  generator and global entropy. Without this feature, [`Rng`] can only be instantiated using
  the [`with_seed`](Rng::with_seed) method.
- `js`: Assumes that WebAssembly targets are being run in a JavaScript environment. See the
  [WebAssembly Notes](#webassembly-notes) section for more information.

# WebAssembly Notes

For non-WASI WASM targets, there is additional sublety to consider when utilizing the global RNG.
By default, `std` targets will use entropy sources in the standard library to seed the global RNG.
However, these sources are not available by default on WASM targets outside of WASI.

If the `js` feature is enabled, this crate will assume that it is running in a JavaScript
environment. At this point, the [`getrandom`] crate will be used in order to access the available
entropy sources and seed the global RNG. If the `js` feature is not enabled, the global RNG will
use a predefined seed.

[`fastrand-contrib`]: https://crates.io/crates/fastrand-contrib
[`getrandom`]: https://crates.io/crates/getrandom

## Types

### Struct `Rng`

A random number generator.

```rust
pub struct Rng(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Rng { /* ... */ }
  ```
  Creates a new random number generator.

- ```rust
  pub fn with_seed(seed: u64) -> Self { /* ... */ }
  ```
  Creates a new random number generator with the initial seed.

- ```rust
  pub fn fork(self: &mut Self) -> Self { /* ... */ }
  ```
  Clones the generator by deterministically deriving a new generator based on the initial

- ```rust
  pub fn alphabetic(self: &mut Self) -> char { /* ... */ }
  ```
  Generates a random `char` in ranges a-z and A-Z.

- ```rust
  pub fn alphanumeric(self: &mut Self) -> char { /* ... */ }
  ```
  Generates a random `char` in ranges a-z, A-Z and 0-9.

- ```rust
  pub fn bool(self: &mut Self) -> bool { /* ... */ }
  ```
  Generates a random `bool`.

- ```rust
  pub fn digit(self: &mut Self, base: u32) -> char { /* ... */ }
  ```
  Generates a random digit in the given `base`.

- ```rust
  pub fn f32(self: &mut Self) -> f32 { /* ... */ }
  ```
  Generates a random `f32` in range `0..1`.

- ```rust
  pub fn f64(self: &mut Self) -> f64 { /* ... */ }
  ```
  Generates a random `f64` in range `0..1`.

- ```rust
  pub fn choose_multiple<I: IntoIterator>(self: &mut Self, source: I, amount: usize) -> Vec<<I as >::Item> { /* ... */ }
  ```
  Collects `amount` values at random from the iterable into a vector.

- ```rust
  pub fn i8</* synthetic */ impl RangeBounds<i8>: RangeBounds<i8>>(self: &mut Self, range: impl RangeBounds<i8>) -> i8 { /* ... */ }
  ```
  Generates a random `i8` in the given range.

- ```rust
  pub fn i16</* synthetic */ impl RangeBounds<i16>: RangeBounds<i16>>(self: &mut Self, range: impl RangeBounds<i16>) -> i16 { /* ... */ }
  ```
  Generates a random `i16` in the given range.

- ```rust
  pub fn i32</* synthetic */ impl RangeBounds<i32>: RangeBounds<i32>>(self: &mut Self, range: impl RangeBounds<i32>) -> i32 { /* ... */ }
  ```
  Generates a random `i32` in the given range.

- ```rust
  pub fn i64</* synthetic */ impl RangeBounds<i64>: RangeBounds<i64>>(self: &mut Self, range: impl RangeBounds<i64>) -> i64 { /* ... */ }
  ```
  Generates a random `i64` in the given range.

- ```rust
  pub fn i128</* synthetic */ impl RangeBounds<i128>: RangeBounds<i128>>(self: &mut Self, range: impl RangeBounds<i128>) -> i128 { /* ... */ }
  ```
  Generates a random `i128` in the given range.

- ```rust
  pub fn isize</* synthetic */ impl RangeBounds<isize>: RangeBounds<isize>>(self: &mut Self, range: impl RangeBounds<isize>) -> isize { /* ... */ }
  ```
  Generates a random `isize` in the given range.

- ```rust
  pub fn lowercase(self: &mut Self) -> char { /* ... */ }
  ```
  Generates a random `char` in range a-z.

- ```rust
  pub fn seed(self: &mut Self, seed: u64) { /* ... */ }
  ```
  Initializes this generator with the given seed.

- ```rust
  pub fn get_seed(self: &Self) -> u64 { /* ... */ }
  ```
  Gives back **current** seed that is being held by this generator.

- ```rust
  pub fn choice<I>(self: &mut Self, iter: I) -> Option<<I as >::Item>
where
    I: IntoIterator,
    <I as >::IntoIter: ExactSizeIterator { /* ... */ }
  ```
  Choose an item from an iterator at random.

- ```rust
  pub fn shuffle<T>(self: &mut Self, slice: &mut [T]) { /* ... */ }
  ```
  Shuffles a slice randomly.

- ```rust
  pub fn fill(self: &mut Self, slice: &mut [u8]) { /* ... */ }
  ```
  Fill a byte slice with random data.

- ```rust
  pub fn u8</* synthetic */ impl RangeBounds<u8>: RangeBounds<u8>>(self: &mut Self, range: impl RangeBounds<u8>) -> u8 { /* ... */ }
  ```
  Generates a random `u8` in the given range.

- ```rust
  pub fn u16</* synthetic */ impl RangeBounds<u16>: RangeBounds<u16>>(self: &mut Self, range: impl RangeBounds<u16>) -> u16 { /* ... */ }
  ```
  Generates a random `u16` in the given range.

- ```rust
  pub fn u32</* synthetic */ impl RangeBounds<u32>: RangeBounds<u32>>(self: &mut Self, range: impl RangeBounds<u32>) -> u32 { /* ... */ }
  ```
  Generates a random `u32` in the given range.

- ```rust
  pub fn u64</* synthetic */ impl RangeBounds<u64>: RangeBounds<u64>>(self: &mut Self, range: impl RangeBounds<u64>) -> u64 { /* ... */ }
  ```
  Generates a random `u64` in the given range.

- ```rust
  pub fn u128</* synthetic */ impl RangeBounds<u128>: RangeBounds<u128>>(self: &mut Self, range: impl RangeBounds<u128>) -> u128 { /* ... */ }
  ```
  Generates a random `u128` in the given range.

- ```rust
  pub fn usize</* synthetic */ impl RangeBounds<usize>: RangeBounds<usize>>(self: &mut Self, range: impl RangeBounds<usize>) -> usize { /* ... */ }
  ```
  Generates a random `usize` in the given range.

- ```rust
  pub fn uppercase(self: &mut Self) -> char { /* ... */ }
  ```
  Generates a random `char` in range A-Z.

- ```rust
  pub fn char</* synthetic */ impl RangeBounds<char>: RangeBounds<char>>(self: &mut Self, range: impl RangeBounds<char>) -> char { /* ... */ }
  ```
  Generates a random `char` in the given range.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Rng { /* ... */ }
    ```
    Clones the generator by creating a new generator with the same seed.

- **Freeze**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Rng) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Rng { /* ... */ }
    ```
    Initialize the `Rng` from the system's random number generator.

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `global_rng::*`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use global_rng::*;
```

