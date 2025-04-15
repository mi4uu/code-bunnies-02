# Crate Documentation

**Version:** 2.6.1

**Format Version:** 43

# Module `subtle`

# subtle [![](https://img.shields.io/crates/v/subtle.svg)](https://crates.io/crates/subtle) [![](https://img.shields.io/badge/dynamic/json.svg?label=docs&uri=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fsubtle%2Fversions&query=%24.versions%5B0%5D.num&colorB=4F74A6)](https://doc.dalek.rs/subtle) [![](https://travis-ci.org/dalek-cryptography/subtle.svg?branch=master)](https://travis-ci.org/dalek-cryptography/subtle)

**Pure-Rust traits and utilities for constant-time cryptographic implementations.**

It consists of a `Choice` type, and a collection of traits using `Choice`
instead of `bool` which are intended to execute in constant-time.  The `Choice`
type is a wrapper around a `u8` that holds a `0` or `1`.

```toml
subtle = "2.6"
```

This crate represents a “best-effort” attempt, since side-channels
are ultimately a property of a deployed cryptographic system
including the hardware it runs on, not just of software.

The traits are implemented using bitwise operations, and should execute in
constant time provided that a) the bitwise operations are constant-time and
b) the bitwise operations are not recognized as a conditional assignment and
optimized back into a branch.

For a compiler to recognize that bitwise operations represent a conditional
assignment, it needs to know that the value used to generate the bitmasks is
really a boolean `i1` rather than an `i8` byte value. In an attempt to
prevent this refinement, the crate tries to hide the value of a `Choice`'s
inner `u8` by passing it through a volatile read. For more information, see
the _About_ section below.

Rust versions from 1.51 or higher have const generics support. You may enable
`const-generics` feautre to have `subtle` traits implemented for arrays `[T; N]`.

Versions prior to `2.2` recommended use of the `nightly` feature to enable an
optimization barrier; this is not required in versions `2.2` and above.

Note: the `subtle` crate contains `debug_assert`s to check invariants during
debug builds. These invariant checks involve secret-dependent branches, and
are not present when compiled in release mode. This crate is intended to be
used in release mode.

## Documentation

Documentation is available [here][docs].

## Minimum Supported Rust Version

Rust **1.41** or higher.

Minimum supported Rust version can be changed in the future, but it will be done with a minor version bump.

## About

This library aims to be the Rust equivalent of Go’s `crypto/subtle` module.

Old versions of the optimization barrier in `impl From<u8> for Choice` were
based on Tim Maclean's [work on `rust-timing-shield`][rust-timing-shield],
which attempts to provide a more comprehensive approach for preventing
software side-channels in Rust code.
From version `2.2`, it was based on Diane Hosfelt and Amber Sprenkels' work on
"Secret Types in Rust".

`subtle` is authored by isis agora lovecruft and Henry de Valence.

## Warning

This code is a low-level library, intended for specific use-cases implementing
cryptographic protocols.  It represents a best-effort attempt to protect
against some software side-channels.  Because side-channel resistance is not a
property of software alone, but of software together with hardware, any such
effort is fundamentally limited.

**USE AT YOUR OWN RISK**

[docs]: https://docs.rs/subtle
[rust-timing-shield]: https://www.chosenplaintext.ca/open-source/rust-timing-shield/security

## Types

### Struct `Choice`

The `Choice` struct represents a choice for use in conditional assignment.

It is a wrapper around a `u8`, which should have the value either `1` (true)
or `0` (false).

The conversion from `u8` to `Choice` passes the value through an optimization
barrier, as a best-effort attempt to prevent the compiler from inferring that
the `Choice` value is a boolean. This strategy is based on Tim Maclean's
[work on `rust-timing-shield`][rust-timing-shield], which attempts to provide
a more comprehensive approach for preventing software side-channels in Rust
code.

The `Choice` struct implements operators for AND, OR, XOR, and NOT, to allow
combining `Choice` values. These operations do not short-circuit.

[rust-timing-shield]:
https://www.chosenplaintext.ca/open-source/rust-timing-shield/security

```rust
pub struct Choice(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn unwrap_u8(self: &Self) -> u8 { /* ... */ }
  ```
  Unwrap the `Choice` wrapper to reveal the underlying `u8`.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: Choice) -> bool { /* ... */ }
    ```
    Convert the `Choice` wrapper into a `bool`, depending on whether

  - ```rust
    fn from(input: u8) -> Choice { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Choice) -> Choice { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Choice { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Choice) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Choice) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Choice) { /* ... */ }
    ```

- **ConstantTimeEq**
  - ```rust
    fn ct_eq(self: &Self, rhs: &Choice) -> Choice { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Choice) -> Choice { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Choice) -> Choice { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Choice { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ConditionallySelectable**
  - ```rust
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `CtOption`

The `CtOption<T>` type represents an optional value similar to the
[`Option<T>`](core::option::Option) type but is intended for
use in constant time APIs.

Any given `CtOption<T>` is either `Some` or `None`, but unlike
`Option<T>` these variants are not exposed. The
[`is_some()`](CtOption::is_some) method is used to determine if
the value is `Some`, and [`unwrap_or()`](CtOption::unwrap_or) and
[`unwrap_or_else()`](CtOption::unwrap_or_else) methods are
provided to access the underlying value. The value can also be
obtained with [`unwrap()`](CtOption::unwrap) but this will panic
if it is `None`.

Functions that are intended to be constant time may not produce
valid results for all inputs, such as square root and inversion
operations in finite field arithmetic. Returning an `Option<T>`
from these functions makes it difficult for the caller to reason
about the result in constant time, and returning an incorrect
value burdens the caller and increases the chance of bugs.

```rust
pub struct CtOption<T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(value: T, is_some: Choice) -> CtOption<T> { /* ... */ }
  ```
  This method is used to construct a new `CtOption<T>` and takes

- ```rust
  pub fn expect(self: Self, msg: &str) -> T { /* ... */ }
  ```
  Returns the contained value, consuming the `self` value.

- ```rust
  pub fn unwrap(self: Self) -> T { /* ... */ }
  ```
  This returns the underlying value but panics if it

- ```rust
  pub fn unwrap_or(self: Self, def: T) -> T
where
    T: ConditionallySelectable { /* ... */ }
  ```
  This returns the underlying value if it is `Some`

- ```rust
  pub fn unwrap_or_else<F>(self: Self, f: F) -> T
where
    T: ConditionallySelectable,
    F: FnOnce() -> T { /* ... */ }
  ```
  This returns the underlying value if it is `Some`

- ```rust
  pub fn is_some(self: &Self) -> Choice { /* ... */ }
  ```
  Returns a true `Choice` if this value is `Some`.

- ```rust
  pub fn is_none(self: &Self) -> Choice { /* ... */ }
  ```
  Returns a true `Choice` if this value is `None`.

- ```rust
  pub fn map<U, F>(self: Self, f: F) -> CtOption<U>
where
    T: Default + ConditionallySelectable,
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Returns a `None` value if the option is `None`, otherwise

- ```rust
  pub fn and_then<U, F>(self: Self, f: F) -> CtOption<U>
where
    T: Default + ConditionallySelectable,
    F: FnOnce(T) -> CtOption<U> { /* ... */ }
  ```
  Returns a `None` value if the option is `None`, otherwise

- ```rust
  pub fn or_else<F>(self: Self, f: F) -> CtOption<T>
where
    T: ConditionallySelectable,
    F: FnOnce() -> CtOption<T> { /* ... */ }
  ```
  Returns `self` if it contains a value, and otherwise returns the result of

- ```rust
  pub fn into_option(self: Self) -> Option<T> { /* ... */ }
  ```
  Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CtOption<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ConditionallySelectable**
  - ```rust
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: CtOption<T>) -> Option<T> { /* ... */ }
    ```
    Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ConstantTimeEq**
  - ```rust
    fn ct_eq(self: &Self, rhs: &CtOption<T>) -> Choice { /* ... */ }
    ```
    Two `CtOption<T>`s are equal if they are both `Some` and

- **Send**
### Struct `BlackBox`

Wrapper type which implements an optimization barrier for all accesses.

```rust
pub struct BlackBox<T: Copy>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(value: T) -> Self { /* ... */ }
  ```
  Constructs a new instance of `BlackBox` which will wrap the specified value.

- ```rust
  pub fn get(self: Self) -> T { /* ... */ }
  ```
  Read the inner value, applying an optimization barrier on access.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BlackBox<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Traits

### Trait `ConstantTimeEq`

**Attributes:**

- `#[allow(unused_attributes)]`

An `Eq`-like trait that produces a `Choice` instead of a `bool`.

# Example

```
use subtle::ConstantTimeEq;
let x: u8 = 5;
let y: u8 = 13;

assert_eq!(x.ct_eq(&y).unwrap_u8(), 0);
assert_eq!(x.ct_eq(&x).unwrap_u8(), 1);
```

```rust
pub trait ConstantTimeEq {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `ct_eq`: Determine if two items are equal.

#### Provided Methods

- ```rust
  fn ct_ne(self: &Self, other: &Self) -> Choice { /* ... */ }
  ```
  Determine if two items are NOT equal.

#### Implementations

This trait is implemented for the following types:

- `[T]` with <T: ConstantTimeEq>
- `Choice`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `usize`
- `isize`
- `cmp::Ordering`
- `CtOption<T>` with <T: ConstantTimeEq>

### Trait `ConditionallySelectable`

**Attributes:**

- `#[allow(unused_attributes)]`

A type which can be conditionally selected in constant time.

This trait also provides generic implementations of conditional
assignment and conditional swaps.

```rust
pub trait ConditionallySelectable: Copy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `conditional_select`: Select `a` or `b` according to `choice`.

#### Provided Methods

- ```rust
  fn conditional_assign(self: &mut Self, other: &Self, choice: Choice) { /* ... */ }
  ```
  Conditionally assign `other` to `self`, according to `choice`.

- ```rust
  fn conditional_swap(a: &mut Self, b: &mut Self, choice: Choice) { /* ... */ }
  ```
  Conditionally swap `self` and `other` if `choice == 1`; otherwise,

#### Implementations

This trait is implemented for the following types:

- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `cmp::Ordering`
- `Choice`
- `CtOption<T>` with <T: ConditionallySelectable>

### Trait `ConditionallyNegatable`

**Attributes:**

- `#[allow(unused_attributes)]`

A type which can be conditionally negated in constant time.

# Note

A generic implementation of `ConditionallyNegatable` is provided
for types `T` which are `ConditionallySelectable` and have `Neg`
implemented on `&T`.

```rust
pub trait ConditionallyNegatable {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `conditional_negate`: Negate `self` if `choice == Choice(1)`; otherwise, leave it

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `ConstantTimeGreater`

A type which can be compared in some manner and be determined to be greater
than another of the same type.

```rust
pub trait ConstantTimeGreater {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `ct_gt`: Determine whether `self > other`.

#### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `cmp::Ordering`

### Trait `ConstantTimeLess`

A type which can be compared in some manner and be determined to be less
than another of the same type.

```rust
pub trait ConstantTimeLess: ConstantTimeEq + ConstantTimeGreater {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn ct_lt(self: &Self, other: &Self) -> Choice { /* ... */ }
  ```
  Determine whether `self < other`.

#### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `cmp::Ordering`

