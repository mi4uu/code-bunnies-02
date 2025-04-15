# Crate Documentation

**Version:** 0.8.5

**Format Version:** 43

# Module `rand`

Utilities for random number generation

Rand provides utilities to generate random numbers, to convert them to
useful types and distributions, and some randomness-related algorithms.

# Quick Start

To get you started quickly, the easiest and highest-level way to get
a random value is to use [`random()`]; alternatively you can use
[`thread_rng()`]. The [`Rng`] trait provides a useful API on all RNGs, while
the [`distributions`] and [`seq`] modules provide further
functionality on top of RNGs.

```
use rand::prelude::*;

if rand::random() { // generates a boolean
    // Try printing a random unicode code point (probably a bad idea)!
    println!("char: {}", rand::random::<char>());
}

let mut rng = rand::thread_rng();
let y: f64 = rng.gen(); // generates a float between 0 and 1

let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
```

# The Book

For the user guide and further documentation, please read
[The Rust Rand Book](https://rust-random.github.io/book).

## Modules

## Module `distributions`

Generating random samples from probability distributions

This module is the home of the [`Distribution`] trait and several of its
implementations. It is the workhorse behind some of the convenient
functionality of the [`Rng`] trait, e.g. [`Rng::gen`] and of course
[`Rng::sample`].

Abstractly, a [probability distribution] describes the probability of
occurrence of each value in its sample space.

More concretely, an implementation of `Distribution<T>` for type `X` is an
algorithm for choosing values from the sample space (a subset of `T`)
according to the distribution `X` represents, using an external source of
randomness (an RNG supplied to the `sample` function).

A type `X` may implement `Distribution<T>` for multiple types `T`.
Any type implementing [`Distribution`] is stateless (i.e. immutable),
but it may have internal parameters set at construction time (for example,
[`Uniform`] allows specification of its sample space as a range within `T`).


# The `Standard` distribution

The [`Standard`] distribution is important to mention. This is the
distribution used by [`Rng::gen`] and represents the "default" way to
produce a random value for many different types, including most primitive
types, tuples, arrays, and a few derived types. See the documentation of
[`Standard`] for more details.

Implementing `Distribution<T>` for [`Standard`] for user types `T` makes it
possible to generate type `T` with [`Rng::gen`], and by extension also
with the [`random`] function.

## Random characters

[`Alphanumeric`] is a simple distribution to sample random letters and
numbers of the `char` type; in contrast [`Standard`] may sample any valid
`char`.


# Uniform numeric ranges

The [`Uniform`] distribution is more flexible than [`Standard`], but also
more specialised: it supports fewer target types, but allows the sample
space to be specified as an arbitrary range within its target type `T`.
Both [`Standard`] and [`Uniform`] are in some sense uniform distributions.

Values may be sampled from this distribution using [`Rng::sample(Range)`] or
by creating a distribution object with [`Uniform::new`],
[`Uniform::new_inclusive`] or `From<Range>`. When the range limits are not
known at compile time it is typically faster to reuse an existing
`Uniform` object than to call [`Rng::sample(Range)`].

User types `T` may also implement `Distribution<T>` for [`Uniform`],
although this is less straightforward than for [`Standard`] (see the
documentation in the [`uniform`] module). Doing so enables generation of
values of type `T` with  [`Rng::sample(Range)`].

## Open and half-open ranges

There are surprisingly many ways to uniformly generate random floats. A
range between 0 and 1 is standard, but the exact bounds (open vs closed)
and accuracy differ. In addition to the [`Standard`] distribution Rand offers
[`Open01`] and [`OpenClosed01`]. See "Floating point implementation" section of
[`Standard`] documentation for more details.

# Non-uniform sampling

Sampling a simple true/false outcome with a given probability has a name:
the [`Bernoulli`] distribution (this is used by [`Rng::gen_bool`]).

For weighted sampling from a sequence of discrete values, use the
[`WeightedIndex`] distribution.

This crate no longer includes other non-uniform distributions; instead
it is recommended that you use either [`rand_distr`] or [`statrs`].


[probability distribution]: https://en.wikipedia.org/wiki/Probability_distribution
[`rand_distr`]: https://crates.io/crates/rand_distr
[`statrs`]: https://crates.io/crates/statrs
[`random`]: crate::random
[`rand_distr`]: https://crates.io/crates/rand_distr
[`statrs`]: https://crates.io/crates/statrs

```rust
pub mod distributions { /* ... */ }
```

### Modules

## Module `uniform`

A distribution uniformly sampling numbers within a given range.

[`Uniform`] is the standard distribution to sample uniformly from a range;
e.g. `Uniform::new_inclusive(1, 6)` can sample integers from 1 to 6, like a
standard die. [`Rng::gen_range`] supports any type supported by
[`Uniform`].

This distribution is provided with support for several primitive types
(all integer and floating-point types) as well as [`std::time::Duration`],
and supports extension to user-defined types via a type-specific *back-end*
implementation.

The types [`UniformInt`], [`UniformFloat`] and [`UniformDuration`] are the
back-ends supporting sampling from primitive integer and floating-point
ranges as well as from [`std::time::Duration`]; these types do not normally
need to be used directly (unless implementing a derived back-end).

# Example usage

```
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

let mut rng = thread_rng();
let side = Uniform::new(-10.0, 10.0);

// sample between 1 and 10 points
for _ in 0..rng.gen_range(1..=10) {
    // sample a point from the square with sides -10 - 10 in two dimensions
    let (x, y) = (rng.sample(side), rng.sample(side));
    println!("Point: {}, {}", x, y);
}
```

# Extending `Uniform` to support a custom type

To extend [`Uniform`] to support your own types, write a back-end which
implements the [`UniformSampler`] trait, then implement the [`SampleUniform`]
helper trait to "register" your back-end. See the `MyF32` example below.

At a minimum, the back-end needs to store any parameters needed for sampling
(e.g. the target range) and implement `new`, `new_inclusive` and `sample`.
Those methods should include an assert to check the range is valid (i.e.
`low < high`). The example below merely wraps another back-end.

The `new`, `new_inclusive` and `sample_single` functions use arguments of
type SampleBorrow<X> in order to support passing in values by reference or
by value. In the implementation of these functions, you can choose to
simply use the reference returned by [`SampleBorrow::borrow`], or you can choose
to copy or clone the value, whatever is appropriate for your type.

```
use rand::prelude::*;
use rand::distributions::uniform::{Uniform, SampleUniform,
        UniformSampler, UniformFloat, SampleBorrow};

struct MyF32(f32);

#[derive(Clone, Copy, Debug)]
struct UniformMyF32(UniformFloat<f32>);

impl UniformSampler for UniformMyF32 {
    type X = MyF32;
    fn new<B1, B2>(low: B1, high: B2) -> Self
        where B1: SampleBorrow<Self::X> + Sized,
              B2: SampleBorrow<Self::X> + Sized
    {
        UniformMyF32(UniformFloat::<f32>::new(low.borrow().0, high.borrow().0))
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
        where B1: SampleBorrow<Self::X> + Sized,
              B2: SampleBorrow<Self::X> + Sized
    {
        UniformMyF32(UniformFloat::<f32>::new_inclusive(
            low.borrow().0,
            high.borrow().0,
        ))
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        MyF32(self.0.sample(rng))
    }
}

impl SampleUniform for MyF32 {
    type Sampler = UniformMyF32;
}

let (low, high) = (MyF32(17.0f32), MyF32(22.0f32));
let uniform = Uniform::new(low, high);
let x = uniform.sample(&mut thread_rng());
```

[`SampleUniform`]: crate::distributions::uniform::SampleUniform
[`UniformSampler`]: crate::distributions::uniform::UniformSampler
[`UniformInt`]: crate::distributions::uniform::UniformInt
[`UniformFloat`]: crate::distributions::uniform::UniformFloat
[`UniformDuration`]: crate::distributions::uniform::UniformDuration
[`SampleBorrow::borrow`]: crate::distributions::uniform::SampleBorrow::borrow

```rust
pub mod uniform { /* ... */ }
```

### Types

#### Struct `Uniform`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`
- `#[<cfg_attr>(feature = "serde1",
serde(bound(serialize = "X::Sampler: Serialize")))]`
- `#[<cfg_attr>(feature = "serde1",
serde(bound(deserialize = "X::Sampler: Deserialize<'de>")))]`

Sample values uniformly between two bounds.

[`Uniform::new`] and [`Uniform::new_inclusive`] construct a uniform
distribution sampling from the given range; these functions may do extra
work up front to make sampling of multiple values faster. If only one sample
from the range is required, [`Rng::gen_range`] can be more efficient.

When sampling from a constant range, many calculations can happen at
compile-time and all methods should be fast; for floating-point ranges and
the full range of integer types this should have comparable performance to
the `Standard` distribution.

Steps are taken to avoid bias which might be present in naive
implementations; for example `rng.gen::<u8>() % 170` samples from the range
`[0, 169]` but is twice as likely to select numbers less than 85 than other
values. Further, the implementations here give more weight to the high-bits
generated by the RNG than the low bits, since with some RNGs the low-bits
are of lower quality than the high bits.

Implementations must sample in `[low, high)` range for
`Uniform::new(low, high)`, i.e., excluding `high`. In particular, care must
be taken to ensure that rounding never results values `< low` or `>= high`.

# Example

```
use rand::distributions::{Distribution, Uniform};

let between = Uniform::from(10..10000);
let mut rng = rand::thread_rng();
let mut sum = 0;
for _ in 0..1000 {
    sum += between.sample(&mut rng);
}
println!("{}", sum);
```

For a single sample, [`Rng::gen_range`] may be preferred:

```
use rand::Rng;

let mut rng = rand::thread_rng();
println!("{}", rng.gen_range(0..10));
```

[`new`]: Uniform::new
[`new_inclusive`]: Uniform::new_inclusive
[`Rng::gen_range`]: Rng::gen_range

```rust
pub struct Uniform<X: SampleUniform>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new<B1, B2>(low: B1, high: B2) -> Uniform<X>
where
    B1: SampleBorrow<X> + Sized,
    B2: SampleBorrow<X> + Sized { /* ... */ }
  ```
  Create a new `Uniform` instance which samples uniformly from the half

- ```rust
  pub fn new_inclusive<B1, B2>(low: B1, high: B2) -> Uniform<X>
where
    B1: SampleBorrow<X> + Sized,
    B2: SampleBorrow<X> + Sized { /* ... */ }
  ```
  Create a new `Uniform` instance which samples uniformly from the closed

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Uniform<X> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Uniform<X>) -> bool { /* ... */ }
    ```

- **Distribution**
  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> X { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(r: ::core::ops::Range<X>) -> Uniform<X> { /* ... */ }
    ```

  - ```rust
    fn from(r: ::core::ops::RangeInclusive<X>) -> Uniform<X> { /* ... */ }
    ```

- **Send**
#### Struct `UniformInt`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

The back-end implementing [`UniformSampler`] for integer types.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

# Implementation notes

For simplicity, we use the same generic struct `UniformInt<X>` for all
integer types `X`. This gives us only one field type, `X`; to store unsigned
values of this size, we take use the fact that these conversions are no-ops.

For a closed range, the number of possible numbers we should generate is
`range = (high - low + 1)`. To avoid bias, we must ensure that the size of
our sample space, `zone`, is a multiple of `range`; other values must be
rejected (by replacing with a new random sample).

As a special case, we use `range = 0` to represent the full range of the
result type (i.e. for `new_inclusive($ty::MIN, $ty::MAX)`).

The optimum `zone` is the largest product of `range` which fits in our
(unsigned) target type. We calculate this by calculating how many numbers we
must reject: `reject = (MAX + 1) % range = (MAX - range + 1) % range`. Any (large)
product of `range` will suffice, thus in `sample_single` we multiply by a
power of 2 via bit-shifting (faster but may cause more rejections).

The smallest integer PRNGs generate is `u32`. For 8- and 16-bit outputs we
use `u32` for our `zone` and samples (because it's not slower and because
it reduces the chance of having to reject a sample). In this case we cannot
store `zone` in the target type since it is too large, however we know
`ints_to_reject < range <= $unsigned::MAX`.

An alternative to using a modulus is widening multiply: After a widening
multiply by `range`, the result is in the high word. Then comparing the low
word against `zone` makes sure our distribution is uniform.

```rust
pub struct UniformInt<X> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UniformSampler**
  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniformInt<X> { /* ... */ }
    ```

- **Copy**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UniformInt<X>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `UniformChar`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

The back-end implementing [`UniformSampler`] for `char`.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

This differs from integer range sampling since the range `0xD800..=0xDFFF`
are used for surrogate pairs in UCS and UTF-16, and consequently are not
valid Unicode code points. We must therefore avoid sampling values in this
range.

```rust
pub struct UniformChar {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **UniformSampler**
  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniformChar { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

#### Struct `UniformFloat`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

The back-end implementing [`UniformSampler`] for floating-point types.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

# Implementation notes

Instead of generating a float in the `[0, 1)` range using [`Standard`], the
`UniformFloat` implementation converts the output of an PRNG itself. This
way one or two steps can be optimized out.

The floats are first converted to a value in the `[1, 2)` interval using a
transmute-based method, and then mapped to the expected range with a
multiply and addition. Values produced this way have what equals 23 bits of
random digits for an `f32`, and 52 for an `f64`.

[`new`]: UniformSampler::new
[`new_inclusive`]: UniformSampler::new_inclusive
[`Standard`]: crate::distributions::Standard

```rust
pub struct UniformFloat<X> {
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
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UniformFloat<X>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UniformSampler**
  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> <Self as >::X { /* ... */ }
    ```

  - ```rust
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniformFloat<X> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `UniformDuration`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

The back-end implementing [`UniformSampler`] for `Duration`.

Unless you are implementing [`UniformSampler`] for your own types, this type
should not be used directly, use [`Uniform`] instead.

```rust
pub struct UniformDuration {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniformDuration { /* ... */ }
    ```

- **UniformSampler**
  - ```rust
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> Duration { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
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
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `SampleUniform`

Helper trait for creating objects using the correct implementation of
[`UniformSampler`] for the sampling type.

See the [module documentation] on how to implement [`Uniform`] range
sampling for a custom type.

[module documentation]: crate::distributions::uniform

```rust
pub trait SampleUniform: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Sampler`: The `UniformSampler` implementation supporting type `X`.

##### Implementations

This trait is implemented for the following types:

- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`
- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `char`
- `f32`
- `f64`
- `core::time::Duration`

#### Trait `UniformSampler`

Helper trait handling actual uniform sampling.

See the [module documentation] on how to implement [`Uniform`] range
sampling for a custom type.

Implementation of [`sample_single`] is optional, and is only useful when
the implementation can be faster than `Self::new(low, high).sample(rng)`.

[module documentation]: crate::distributions::uniform
[`sample_single`]: UniformSampler::sample_single

```rust
pub trait UniformSampler: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `X`: The type sampled by this implementation.

###### Required Methods

- `new`: Construct self, with inclusive lower bound and exclusive upper bound
- `new_inclusive`: Construct self, with inclusive bounds `[low, high]`.
- `sample`: Sample a value.

##### Provided Methods

- ```rust
  fn sample_single<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
  ```
  Sample a single value uniformly from a range with inclusive lower bound

- ```rust
  fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> <Self as >::X
where
    B1: SampleBorrow<<Self as >::X> + Sized,
    B2: SampleBorrow<<Self as >::X> + Sized { /* ... */ }
  ```
  Sample a single value uniformly from a range with inclusive lower bound

##### Implementations

This trait is implemented for the following types:

- `UniformInt<i8>`
- `UniformInt<i16>`
- `UniformInt<i32>`
- `UniformInt<i64>`
- `UniformInt<i128>`
- `UniformInt<isize>`
- `UniformInt<u8>`
- `UniformInt<u16>`
- `UniformInt<u32>`
- `UniformInt<u64>`
- `UniformInt<usize>`
- `UniformInt<u128>`
- `UniformChar`
- `UniformFloat<f32>`
- `UniformFloat<f64>`
- `UniformDuration`

#### Trait `SampleBorrow`

Helper trait similar to [`Borrow`] but implemented
only for SampleUniform and references to SampleUniform in
order to resolve ambiguity issues.

[`Borrow`]: std::borrow::Borrow

```rust
pub trait SampleBorrow<Borrowed> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `borrow`: Immutably borrows from an owned value. See [`Borrow::borrow`]

##### Implementations

This trait is implemented for the following types:

- `Borrowed` with <Borrowed>
- `&''a Borrowed` with <''a, Borrowed>

#### Trait `SampleRange`

Range that supports generating a single sample efficiently.

Any type implementing this trait can be used to specify the sampled range
for `Rng::gen_range`.

```rust
pub trait SampleRange<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `sample_single`: Generate a sample from the given range.
- `is_empty`: Check whether the range is empty.

##### Implementations

This trait is implemented for the following types:

- `core::ops::Range<T>` with <T: SampleUniform + PartialOrd>
- `core::ops::RangeInclusive<T>` with <T: SampleUniform + PartialOrd>

## Module `weighted`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "alloc")))]`

**⚠️ Deprecated since 0.8.0**: use rand::distributions::{WeightedIndex, WeightedError} instead

Weighted index sampling

This module is deprecated. Use [`crate::distributions::WeightedIndex`] and
[`crate::distributions::WeightedError`] instead.

```rust
pub mod weighted { /* ... */ }
```

### Modules

## Module `alias_method`

**Attributes:**

- `#[allow(missing_docs)]`

**⚠️ Deprecated since 0.8.0**: moved to rand_distr crate

```rust
pub mod alias_method { /* ... */ }
```

### Types

#### Struct `WeightedIndex`

**⚠️ Deprecated since 0.8.0**: moved to rand_distr crate

```rust
pub struct WeightedIndex<W: Weight> {
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
  pub fn new(_weights: Vec<W>) -> Result<Self, WeightedError> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `Weight`

**⚠️ Deprecated since 0.8.0**: moved to rand_distr crate

```rust
pub trait Weight {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `f64`
- `f32`
- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `u128`
- `i128`

### Re-exports

#### Re-export `WeightedIndex`

**⚠️ Deprecated since 0.8.0**: use rand::distributions::{WeightedIndex, WeightedError} instead

```rust
pub use super::WeightedIndex;
```

#### Re-export `WeightedError`

**⚠️ Deprecated since 0.8.0**: use rand::distributions::{WeightedIndex, WeightedError} instead

```rust
pub use super::WeightedError;
```

### Types

#### Struct `Standard`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1",
derive(serde::Serialize, serde::Deserialize))]`

A generic random value distribution, implemented for many primitive types.
Usually generates values with a numerically uniform distribution, and with a
range appropriate to the type.

## Provided implementations

Assuming the provided `Rng` is well-behaved, these implementations
generate values with the following ranges and distributions:

* Integers (`i32`, `u32`, `isize`, `usize`, etc.): Uniformly distributed
  over all values of the type.
* `char`: Uniformly distributed over all Unicode scalar values, i.e. all
  code points in the range `0...0x10_FFFF`, except for the range
  `0xD800...0xDFFF` (the surrogate code points). This includes
  unassigned/reserved code points.
* `bool`: Generates `false` or `true`, each with probability 0.5.
* Floating point types (`f32` and `f64`): Uniformly distributed in the
  half-open range `[0, 1)`. See notes below.
* Wrapping integers (`Wrapping<T>`), besides the type identical to their
  normal integer variants.

The `Standard` distribution also supports generation of the following
compound types where all component types are supported:

*   Tuples (up to 12 elements): each element is generated sequentially.
*   Arrays (up to 32 elements): each element is generated sequentially;
    see also [`Rng::fill`] which supports arbitrary array length for integer
    and float types and tends to be faster for `u32` and smaller types.
    When using `rustc` ≥ 1.51, enable the `min_const_gen` feature to support
    arrays larger than 32 elements.
    Note that [`Rng::fill`] and `Standard`'s array support are *not* equivalent:
    the former is optimised for integer types (using fewer RNG calls for
    element types smaller than the RNG word size), while the latter supports
    any element type supported by `Standard`.
*   `Option<T>` first generates a `bool`, and if true generates and returns
    `Some(value)` where `value: T`, otherwise returning `None`.

## Custom implementations

The [`Standard`] distribution may be implemented for user types as follows:

```
# #![allow(dead_code)]
use rand::Rng;
use rand::distributions::{Distribution, Standard};

struct MyF32 {
    x: f32,
}

impl Distribution<MyF32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyF32 {
        MyF32 { x: rng.gen() }
    }
}
```

## Example usage
```
use rand::prelude::*;
use rand::distributions::Standard;

let val: f32 = StdRng::from_entropy().sample(Standard);
println!("f32 from [0, 1): {}", val);
```

# Floating point implementation
The floating point implementations for `Standard` generate a random value in
the half-open interval `[0, 1)`, i.e. including 0 but not 1.

All values that can be generated are of the form `n * ε/2`. For `f32`
the 24 most significant random bits of a `u32` are used and for `f64` the
53 most significant bits of a `u64` are used. The conversion uses the
multiplicative method: `(rng.gen::<$uty>() >> N) as $ty * (ε/2)`.

See also: [`Open01`] which samples from `(0, 1)`, [`OpenClosed01`] which
samples from `(0, 1]` and `Rng::gen_range(0..1)` which also samples from
`[0, 1)`. Note that `Open01` uses transmute-based methods which yield 1 bit
less precision but may perform faster on some architectures (on modern Intel
CPUs all methods have approximately equal performance).

[`Uniform`]: uniform::Uniform

```rust
pub struct Standard;
```

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DistString**
  - ```rust
    fn append_string<R: Rng + ?Sized>(self: &Self, rng: &mut R, s: &mut String, len: usize) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Distribution**
  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> f32 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> f64 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> u8 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> u16 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> u32 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> u64 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> u128 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> usize { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> i8 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> i16 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> i32 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> i64 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> i128 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> isize { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroU8 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroU16 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroU32 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroU64 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroU128 { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> NonZeroUsize { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> char { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> bool { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _: &mut R) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G, H) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G, H, I) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G, H, I, J) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G, H, I, J, K) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> (A, B, C, D, E, F, G, H, I, J, K, L) { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 0] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 1] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 2] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 3] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 4] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 5] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 6] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 7] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 8] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 9] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 10] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 11] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 12] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 13] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 14] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 15] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 16] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 17] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 18] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 19] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 20] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 21] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 22] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 23] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 24] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 25] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 26] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 27] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 28] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 29] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 30] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 31] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, _rng: &mut R) -> [T; 32] { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn sample<R: Rng + ?Sized>(self: &Self, rng: &mut R) -> Wrapping<T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Standard { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `Bernoulli`

```rust
pub use self::bernoulli::Bernoulli;
```

#### Re-export `BernoulliError`

```rust
pub use self::bernoulli::BernoulliError;
```

#### Re-export `Distribution`

```rust
pub use self::distribution::Distribution;
```

#### Re-export `DistIter`

```rust
pub use self::distribution::DistIter;
```

#### Re-export `DistMap`

```rust
pub use self::distribution::DistMap;
```

#### Re-export `DistString`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::distribution::DistString;
```

#### Re-export `Open01`

```rust
pub use self::float::Open01;
```

#### Re-export `OpenClosed01`

```rust
pub use self::float::OpenClosed01;
```

#### Re-export `Alphanumeric`

```rust
pub use self::other::Alphanumeric;
```

#### Re-export `Slice`

```rust
pub use self::slice::Slice;
```

#### Re-export `Uniform`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::uniform::Uniform;
```

#### Re-export `WeightedError`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::weighted_index::WeightedError;
```

#### Re-export `WeightedIndex`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::weighted_index::WeightedIndex;
```

## Module `prelude`

Convenience re-export of common members

Like the standard library's prelude, this module simplifies importing of
common items. Unlike the standard prelude, the contents of this module must
be imported manually:

```
use rand::prelude::*;
# let mut r = StdRng::from_rng(thread_rng()).unwrap();
# let _: f32 = r.gen();
```

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `Distribution`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::distributions::Distribution;
```

#### Re-export `SmallRng`

**Attributes:**

- `#[<cfg>(feature = "small_rng")]`
- `#[doc(no_inline)]`

```rust
pub use crate::rngs::SmallRng;
```

#### Re-export `StdRng`

**Attributes:**

- `#[<cfg>(feature = "std_rng")]`
- `#[doc(no_inline)]`

```rust
pub use crate::rngs::StdRng;
```

#### Re-export `ThreadRng`

**Attributes:**

- `#[doc(no_inline)]`
- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`

```rust
pub use crate::rngs::ThreadRng;
```

#### Re-export `IteratorRandom`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::seq::IteratorRandom;
```

#### Re-export `SliceRandom`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::seq::SliceRandom;
```

#### Re-export `random`

**Attributes:**

- `#[doc(no_inline)]`
- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`

```rust
pub use crate::random;
```

#### Re-export `thread_rng`

**Attributes:**

- `#[doc(no_inline)]`
- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`

```rust
pub use crate::thread_rng;
```

#### Re-export `CryptoRng`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::CryptoRng;
```

#### Re-export `Rng`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::Rng;
```

#### Re-export `RngCore`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::RngCore;
```

#### Re-export `SeedableRng`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::SeedableRng;
```

## Module `rngs`

Random number generators and adapters

## Background: Random number generators (RNGs)

Computers cannot produce random numbers from nowhere. We classify
random number generators as follows:

-   "True" random number generators (TRNGs) use hard-to-predict data sources
    (e.g. the high-resolution parts of event timings and sensor jitter) to
    harvest random bit-sequences, apply algorithms to remove bias and
    estimate available entropy, then combine these bits into a byte-sequence
    or an entropy pool. This job is usually done by the operating system or
    a hardware generator (HRNG).
-   "Pseudo"-random number generators (PRNGs) use algorithms to transform a
    seed into a sequence of pseudo-random numbers. These generators can be
    fast and produce well-distributed unpredictable random numbers (or not).
    They are usually deterministic: given algorithm and seed, the output
    sequence can be reproduced. They have finite period and eventually loop;
    with many algorithms this period is fixed and can be proven sufficiently
    long, while others are chaotic and the period depends on the seed.
-   "Cryptographically secure" pseudo-random number generators (CSPRNGs)
    are the sub-set of PRNGs which are secure. Security of the generator
    relies both on hiding the internal state and using a strong algorithm.

## Traits and functionality

All RNGs implement the [`RngCore`] trait, as a consequence of which the
[`Rng`] extension trait is automatically implemented. Secure RNGs may
additionally implement the [`CryptoRng`] trait.

All PRNGs require a seed to produce their random number sequence. The
[`SeedableRng`] trait provides three ways of constructing PRNGs:

-   `from_seed` accepts a type specific to the PRNG
-   `from_rng` allows a PRNG to be seeded from any other RNG
-   `seed_from_u64` allows any PRNG to be seeded from a `u64` insecurely
-   `from_entropy` securely seeds a PRNG from fresh entropy

Use the [`rand_core`] crate when implementing your own RNGs.

## Our generators

This crate provides several random number generators:

-   [`OsRng`] is an interface to the operating system's random number
    source. Typically the operating system uses a CSPRNG with entropy
    provided by a TRNG and some type of on-going re-seeding.
-   [`ThreadRng`], provided by the [`thread_rng`] function, is a handle to a
    thread-local CSPRNG with periodic seeding from [`OsRng`]. Because this
    is local, it is typically much faster than [`OsRng`]. It should be
    secure, though the paranoid may prefer [`OsRng`].
-   [`StdRng`] is a CSPRNG chosen for good performance and trust of security
    (based on reviews, maturity and usage). The current algorithm is ChaCha12,
    which is well established and rigorously analysed.
    [`StdRng`] provides the algorithm used by [`ThreadRng`] but without
    periodic reseeding.
-   [`SmallRng`] is an **insecure** PRNG designed to be fast, simple, require
    little memory, and have good output quality.

The algorithms selected for [`StdRng`] and [`SmallRng`] may change in any
release and may be platform-dependent, therefore they should be considered
**not reproducible**.

## Additional generators

**TRNGs**: The [`rdrand`] crate provides an interface to the RDRAND and
RDSEED instructions available in modern Intel and AMD CPUs.
The [`rand_jitter`] crate provides a user-space implementation of
entropy harvesting from CPU timer jitter, but is very slow and has
[security issues](https://github.com/rust-random/rand/issues/699).

**PRNGs**: Several companion crates are available, providing individual or
families of PRNG algorithms. These provide the implementations behind
[`StdRng`] and [`SmallRng`] but can also be used directly, indeed *should*
be used directly when **reproducibility** matters.
Some suggestions are: [`rand_chacha`], [`rand_pcg`], [`rand_xoshiro`].
A full list can be found by searching for crates with the [`rng` tag].

[`Rng`]: crate::Rng
[`RngCore`]: crate::RngCore
[`CryptoRng`]: crate::CryptoRng
[`SeedableRng`]: crate::SeedableRng
[`thread_rng`]: crate::thread_rng
[`rdrand`]: https://crates.io/crates/rdrand
[`rand_jitter`]: https://crates.io/crates/rand_jitter
[`rand_chacha`]: https://crates.io/crates/rand_chacha
[`rand_pcg`]: https://crates.io/crates/rand_pcg
[`rand_xoshiro`]: https://crates.io/crates/rand_xoshiro
[`rng` tag]: https://crates.io/keywords/rng

```rust
pub mod rngs { /* ... */ }
```

### Modules

## Module `adapter`

**Attributes:**

- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "std")))]`
- `#[<cfg>(feature = "std")]`

Wrappers / adapters forming RNGs

```rust
pub mod adapter { /* ... */ }
```

### Re-exports

#### Re-export `ReadError`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::read::ReadError;
```

#### Re-export `ReadRng`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::read::ReadRng;
```

#### Re-export `ReseedingRng`

```rust
pub use self::reseeding::ReseedingRng;
```

## Module `mock`

Mock random number generator

```rust
pub mod mock { /* ... */ }
```

### Types

#### Struct `StepRng`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

A simple implementation of `RngCore` for testing purposes.

This generates an arithmetic sequence (i.e. adds a constant each step)
over a `u64` number, using wrapping arithmetic. If the increment is 0
the generator yields a constant.

```
use rand::Rng;
use rand::rngs::mock::StepRng;

let mut my_rng = StepRng::new(2, 1);
let sample: [u64; 3] = my_rng.gen();
assert_eq!(sample, [2, 3, 4]);
```

```rust
pub struct StepRng {
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
  pub fn new(initial: u64, increment: u64) -> Self { /* ... */ }
  ```
  Create a `StepRng`, yielding an arithmetic sequence starting with

###### Trait Implementations

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **RefUnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Rng**
- **UnwindSafe**
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

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StepRng { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StepRng) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Re-exports

#### Re-export `SmallRng`

**Attributes:**

- `#[<cfg>(feature = "small_rng")]`

```rust
pub use self::small::SmallRng;
```

#### Re-export `StdRng`

**Attributes:**

- `#[<cfg>(feature = "std_rng")]`

```rust
pub use self::std::StdRng;
```

#### Re-export `ThreadRng`

**Attributes:**

- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`

```rust
pub use self::thread::ThreadRng;
```

#### Re-export `OsRng`

**Attributes:**

- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "getrandom")))]`
- `#[<cfg>(feature = "getrandom")]`

```rust
pub use rand_core::OsRng;
```

## Module `seq`

Sequence-related functionality

This module provides:

*   [`SliceRandom`] slice sampling and mutation
*   [`IteratorRandom`] iterator sampling
*   [`index::sample`] low-level API to choose multiple indices from
    `0..length`

Also see:

*   [`crate::distributions::WeightedIndex`] distribution which provides
    weighted index sampling.

In order to make results reproducible across 32-64 bit architectures, all
`usize` indices are sampled as a `u32` where possible (also providing a
small performance boost in some cases).

```rust
pub mod seq { /* ... */ }
```

### Modules

## Module `index`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "alloc")))]`

Low-level API for sampling indices

```rust
pub mod index { /* ... */ }
```

### Types

#### Enum `IndexVec`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1", derive(Serialize, Deserialize))]`

A vector of indices.

Multiple internal representations are possible.

```rust
pub enum IndexVec {
    // Some variants omitted
}
```

##### Variants

*Note: Some variants have been omitted because they are private or hidden.*

##### Implementations

###### Methods

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of indices

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the length is 0.

- ```rust
  pub fn index(self: &Self, index: usize) -> usize { /* ... */ }
  ```
  Return the value at the given `index`.

- ```rust
  pub fn into_vec(self: Self) -> Vec<usize> { /* ... */ }
  ```
  Return result as a `Vec<usize>`. Conversion may or may not be trivial.

- ```rust
  pub fn iter(self: &Self) -> IndexVecIter<''_> { /* ... */ }
  ```
  Iterate over the indices as a sequence of `usize` values

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IndexVec) -> bool { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IndexVecIntoIter { /* ... */ }
    ```
    Convert into an iterator over the indices as a sequence of `usize` values

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IndexVec { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

  - ```rust
    fn from(v: Vec<u32>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(v: Vec<usize>) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `IndexVecIter`

Return type of `IndexVec::iter`.

```rust
pub enum IndexVecIter<''a> {
    // Some variants omitted
}
```

##### Variants

*Note: Some variants have been omitted because they are private or hidden.*

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
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

- **IteratorRandom**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ExactSizeIterator**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Enum `IndexVecIntoIter`

Return type of `IndexVec::into_iter`.

```rust
pub enum IndexVecIntoIter {
    // Some variants omitted
}
```

##### Variants

*Note: Some variants have been omitted because they are private or hidden.*

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IteratorRandom**
- **Sync**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IndexVecIntoIter { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `sample`

Randomly sample exactly `amount` distinct indices from `0..length`, and
return them in random order (fully shuffled).

This method is used internally by the slice sampling methods, but it can
sometimes be useful to have the indices themselves so this is provided as
an alternative.

The implementation used is not specified; we automatically select the
fastest available algorithm for the `length` and `amount` parameters
(based on detailed profiling on an Intel Haswell CPU). Roughly speaking,
complexity is `O(amount)`, except that when `amount` is small, performance
is closer to `O(amount^2)`, and when `length` is close to `amount` then
`O(length)`.

Note that performance is significantly better over `u32` indices than over
`u64` indices. Because of this we hide the underlying type behind an
abstraction, `IndexVec`.

If an allocation-free `no_std` function is required, it is suggested
to adapt the internal `sample_floyd` implementation.

Panics if `amount > length`.

```rust
pub fn sample<R>(rng: &mut R, length: usize, amount: usize) -> IndexVec
where
    R: Rng + ?Sized { /* ... */ }
```

#### Function `sample_weighted`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "std")))]`

Randomly sample exactly `amount` distinct indices from `0..length`, and
return them in an arbitrary order (there is no guarantee of shuffling or
ordering). The weights are to be provided by the input function `weights`,
which will be called once for each index.

This method is used internally by the slice sampling methods, but it can
sometimes be useful to have the indices themselves so this is provided as
an alternative.

This implementation uses `O(length + amount)` space and `O(length)` time
if the "nightly" feature is enabled, or `O(length)` space and
`O(length + amount * log length)` time otherwise.

Panics if `amount > length`.

```rust
pub fn sample_weighted<R, F, X>(rng: &mut R, length: usize, weight: F, amount: usize) -> Result<IndexVec, crate::distributions::WeightedError>
where
    R: Rng + ?Sized,
    F: Fn(usize) -> X,
    X: Into<f64> { /* ... */ }
```

### Types

#### Struct `SliceChooseIter`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[<cfg_attr>(doc_cfg, doc(cfg(feature = "alloc")))]`

An iterator over multiple slice elements.

This struct is created by
[`SliceRandom::choose_multiple`](trait.SliceRandom.html#tymethod.choose_multiple).

```rust
pub struct SliceChooseIter<''a, S: ?Sized + ''a, T: ''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Freeze**
- **IteratorRandom**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
### Traits

#### Trait `SliceRandom`

Extension trait on slices, providing random mutation and sampling methods.

This trait is implemented on all `[T]` slice types, providing several
methods for choosing and shuffling elements. You must `use` this trait:

```
use rand::seq::SliceRandom;

let mut rng = rand::thread_rng();
let mut bytes = "Hello, random!".to_string().into_bytes();
bytes.shuffle(&mut rng);
let str = String::from_utf8(bytes).unwrap();
println!("{}", str);
```
Example output (non-deterministic):
```none
l,nmroHado !le
```

```rust
pub trait SliceRandom {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Item`: The element type.

###### Required Methods

- `choose`: Returns a reference to one random element of the slice, or `None` if the
- `choose_mut`: Returns a mutable reference to one random element of the slice, or
- `choose_multiple`: Chooses `amount` elements from the slice at random, without repetition,
- `choose_weighted`: Similar to [`choose`], but where the likelihood of each outcome may be
- `choose_weighted_mut`: Similar to [`choose_mut`], but where the likelihood of each outcome may
- `choose_multiple_weighted`: Similar to [`choose_multiple`], but where the likelihood of each element's
- `shuffle`: Shuffle a mutable slice in place.
- `partial_shuffle`: Shuffle a slice in place, but exit early.

##### Implementations

This trait is implemented for the following types:

- `[T]` with <T>

#### Trait `IteratorRandom`

Extension trait on iterators, providing random sampling methods.

This trait is implemented on all iterators `I` where `I: Iterator + Sized`
and provides methods for
choosing one or more elements. You must `use` this trait:

```
use rand::seq::IteratorRandom;

let mut rng = rand::thread_rng();

let faces = "😀😎😐😕😠😢";
println!("I am {}!", faces.chars().choose(&mut rng).unwrap());
```
Example output (non-deterministic):
```none
I am 😀!
```

```rust
pub trait IteratorRandom: Iterator + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn choose<R>(self: Self, rng: &mut R) -> Option<<Self as >::Item>
where
    R: Rng + ?Sized { /* ... */ }
  ```
  Choose one element at random from the iterator.

- ```rust
  fn choose_stable<R>(self: Self, rng: &mut R) -> Option<<Self as >::Item>
where
    R: Rng + ?Sized { /* ... */ }
  ```
  Choose one element at random from the iterator.

- ```rust
  fn choose_multiple_fill<R>(self: Self, rng: &mut R, buf: &mut [<Self as >::Item]) -> usize
where
    R: Rng + ?Sized { /* ... */ }
  ```
  Collects values at random from the iterator into a supplied buffer

- ```rust
  fn choose_multiple<R>(self: Self, rng: &mut R, amount: usize) -> Vec<<Self as >::Item>
where
    R: Rng + ?Sized { /* ... */ }
  ```
  Collects `amount` values at random from the iterator into a vector.

##### Implementations

This trait is implemented for the following types:

- `I` with <I>

## Functions

### Function `random`

**Attributes:**

- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`
- `#[<cfg_attr>(doc_cfg, doc(cfg(all(feature = "std", feature = "std_rng"))))]`
- `#[inline]`

Generates a random value using the thread-local random number generator.

This is simply a shortcut for `thread_rng().gen()`. See [`thread_rng`] for
documentation of the entropy source and [`Standard`] for documentation of
distributions and type-specific generation.

# Provided implementations

The following types have provided implementations that
generate values with the following ranges and distributions:

* Integers (`i32`, `u32`, `isize`, `usize`, etc.): Uniformly distributed
  over all values of the type.
* `char`: Uniformly distributed over all Unicode scalar values, i.e. all
  code points in the range `0...0x10_FFFF`, except for the range
  `0xD800...0xDFFF` (the surrogate code points). This includes
  unassigned/reserved code points.
* `bool`: Generates `false` or `true`, each with probability 0.5.
* Floating point types (`f32` and `f64`): Uniformly distributed in the
  half-open range `[0, 1)`. See notes below.
* Wrapping integers (`Wrapping<T>`), besides the type identical to their
  normal integer variants.

Also supported is the generation of the following
compound types where all component types are supported:

*   Tuples (up to 12 elements): each element is generated sequentially.
*   Arrays (up to 32 elements): each element is generated sequentially;
    see also [`Rng::fill`] which supports arbitrary array length for integer
    types and tends to be faster for `u32` and smaller types.
*   `Option<T>` first generates a `bool`, and if true generates and returns
    `Some(value)` where `value: T`, otherwise returning `None`.

# Examples

```
let x = rand::random::<u8>();
println!("{}", x);

let y = rand::random::<f64>();
println!("{}", y);

if rand::random() { // generates a boolean
    println!("Better lucky than good!");
}
```

If you're calling `random()` in a loop, caching the generator as in the
following example can increase performance.

```
use rand::Rng;

let mut v = vec![1, 2, 3];

for x in v.iter_mut() {
    *x = rand::random()
}

// can be made faster by caching thread_rng

let mut rng = rand::thread_rng();

for x in v.iter_mut() {
    *x = rng.gen();
}
```

[`Standard`]: distributions::Standard

```rust
pub fn random<T>() -> T
where
    crate::distributions::Standard: Distribution<T> { /* ... */ }
```

## Re-exports

### Re-export `CryptoRng`

```rust
pub use rand_core::CryptoRng;
```

### Re-export `Error`

```rust
pub use rand_core::Error;
```

### Re-export `RngCore`

```rust
pub use rand_core::RngCore;
```

### Re-export `SeedableRng`

```rust
pub use rand_core::SeedableRng;
```

### Re-export `thread_rng`

**Attributes:**

- `#[<cfg>(all(feature = "std", feature = "std_rng"))]`

```rust
pub use crate::rngs::thread::thread_rng;
```

### Re-export `Fill`

```rust
pub use rng::Fill;
```

### Re-export `Rng`

```rust
pub use rng::Rng;
```

