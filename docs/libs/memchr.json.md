# Crate Documentation

**Version:** 2.7.4

**Format Version:** 43

# Module `memchr`

This library provides heavily optimized routines for string search primitives.

# Overview

This section gives a brief high level overview of what this crate offers.

* The top-level module provides routines for searching for 1, 2 or 3 bytes
  in the forward or reverse direction. When searching for more than one byte,
  positions are considered a match if the byte at that position matches any
  of the bytes.
* The [`memmem`] sub-module provides forward and reverse substring search
  routines.

In all such cases, routines operate on `&[u8]` without regard to encoding. This
is exactly what you want when searching either UTF-8 or arbitrary bytes.

# Example: using `memchr`

This example shows how to use `memchr` to find the first occurrence of `z` in
a haystack:

```
use memchr::memchr;

let haystack = b"foo bar baz quuz";
assert_eq!(Some(10), memchr(b'z', haystack));
```

# Example: matching one of three possible bytes

This examples shows how to use `memrchr3` to find occurrences of `a`, `b` or
`c`, starting at the end of the haystack.

```
use memchr::memchr3_iter;

let haystack = b"xyzaxyzbxyzc";

let mut it = memchr3_iter(b'a', b'b', b'c', haystack).rev();
assert_eq!(Some(11), it.next());
assert_eq!(Some(7), it.next());
assert_eq!(Some(3), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches

This example shows how to use the [`memmem`] sub-module to find occurrences of
a substring in a haystack.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher to be
measurable in some workloads. In cases where the same needle is used to search
many haystacks, it is possible to do construction once and thus to avoid it for
subsequent searches. This can be done with a [`memmem::Finder`]:

```
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

# Why use this crate?

At first glance, the APIs provided by this crate might seem weird. Why provide
a dedicated routine like `memchr` for something that could be implemented
clearly and trivially in one line:

```
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}
```

Or similarly, why does this crate provide substring search routines when Rust's
core library already provides them?

```
fn search(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}
```

The primary reason for both of them to exist is performance. When it comes to
performance, at a high level at least, there are two primary ways to look at
it:

* **Throughput**: For this, think about it as, "given some very large haystack
  and a byte that never occurs in that haystack, how long does it take to
  search through it and determine that it, in fact, does not occur?"
* **Latency**: For this, think about it as, "given a tiny haystack---just a
  few bytes---how long does it take to determine if a byte is in it?"

The `memchr` routine in this crate has _slightly_ worse latency than the
solution presented above, however, its throughput can easily be over an
order of magnitude faster. This is a good general purpose trade off to make.
You rarely lose, but often gain big.

**NOTE:** The name `memchr` comes from the corresponding routine in `libc`. A
key advantage of using this library is that its performance is not tied to its
quality of implementation in the `libc` you happen to be using, which can vary
greatly from platform to platform.

But what about substring search? This one is a bit more complicated. The
primary reason for its existence is still indeed performance, but it's also
useful because Rust's core library doesn't actually expose any substring
search routine on arbitrary bytes. The only substring search routine that
exists works exclusively on valid UTF-8.

So if you have valid UTF-8, is there a reason to use this over the standard
library substring search routine? Yes. This routine is faster on almost every
metric, including latency. The natural question then, is why isn't this
implementation in the standard library, even if only for searching on UTF-8?
The reason is that the implementation details for using SIMD in the standard
library haven't quite been worked out yet.

**NOTE:** Currently, only `x86_64`, `wasm32` and `aarch64` targets have vector
accelerated implementations of `memchr` (and friends) and `memmem`.

# Crate features

* **std** - When enabled (the default), this will permit features specific to
the standard library. Currently, the only thing used from the standard library
is runtime SIMD CPU feature detection. This means that this feature must be
enabled to get AVX2 accelerated routines on `x86_64` targets without enabling
the `avx2` feature at compile time, for example. When `std` is not enabled,
this crate will still attempt to use SSE2 accelerated routines on `x86_64`. It
will also use AVX2 accelerated routines when the `avx2` feature is enabled at
compile time. In general, enable this feature if you can.
* **alloc** - When enabled (the default), APIs in this crate requiring some
kind of allocation will become available. For example, the
[`memmem::Finder::into_owned`](crate::memmem::Finder::into_owned) API and the
[`arch::all::shiftor`](crate::arch::all::shiftor) substring search
implementation. Otherwise, this crate is designed from the ground up to be
usable in core-only contexts, so the `alloc` feature doesn't add much
currently. Notably, disabling `std` but enabling `alloc` will **not** result
in the use of AVX2 on `x86_64` targets unless the `avx2` feature is enabled
at compile time. (With `std` enabled, AVX2 can be used even without the `avx2`
feature enabled at compile time by way of runtime CPU feature detection.)
* **logging** - When enabled (disabled by default), the `log` crate is used
to emit log messages about what kinds of `memchr` and `memmem` algorithms
are used. Namely, both `memchr` and `memmem` have a number of different
implementation choices depending on the target and CPU, and the log messages
can help show what specific implementations are being used. Generally, this is
useful for debugging performance issues.
* **libc** - **DEPRECATED**. Previously, this enabled the use of the target's
`memchr` function from whatever `libc` was linked into the program. This
feature is now a no-op because this crate's implementation of `memchr` should
now be sufficiently fast on a number of platforms that `libc` should no longer
be needed. (This feature is somewhat of a holdover from this crate's origins.
Originally, this crate was literally just a safe wrapper function around the
`memchr` function from `libc`.)

## Modules

## Module `arch`

A module with low-level architecture dependent routines.

These routines are useful as primitives for tasks not covered by the higher
level crate API.

```rust
pub mod arch { /* ... */ }
```

### Modules

## Module `all`

Contains architecture independent routines.

These routines are often used as a "fallback" implementation when the more
specialized architecture dependent routines are unavailable.

```rust
pub mod all { /* ... */ }
```

### Modules

## Module `memchr`

Provides architecture independent implementations of `memchr` and friends.

The main types in this module are [`One`], [`Two`] and [`Three`]. They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers
are typically slower than hand-coded vector routines accomplishing the same
task, but are also typically faster than naive scalar code. These routines
effectively work by treating a `usize` as a vector of 8-bit lanes, and thus
achieves some level of data parallelism even without explicit vector support.

The `One` searcher also provides a [`One::count`] routine for efficiently
counting the number of times a single byte occurs in a haystack. This is
useful, for example, for counting the number of lines in a haystack. This
routine exists because it is usually faster, especially with a high match
count, then using [`One::find`] repeatedly. ([`OneIter`] specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.

```rust
pub mod memchr { /* ... */ }
```

### Types

#### Struct `One`

Finds all occurrences of a single byte in a haystack.

```rust
pub struct One {
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
  pub fn new(needle: u8) -> One { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the byte given.

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of the needle in the given haystack. If no

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of the needle in the given haystack. If no

- ```rust
  pub fn count(self: &Self, haystack: &[u8]) -> usize { /* ... */ }
  ```
  Counts all occurrences of this byte in the given haystack.

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize { /* ... */ }
  ```
  Counts all occurrences of this byte in the given haystack represented

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> OneIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of the needle byte in the

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> One { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `OneIter`

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`One::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct OneIter<''a, ''h> {
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

- **Send**
- **Sync**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OneIter<''a, ''h> { /* ... */ }
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

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Two`

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

```rust
pub struct Two {
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
  pub fn new(needle1: u8, needle2: u8) -> Two { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the two needle bytes

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of one of the needle bytes in the given

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of one of the needle bytes in the given

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> TwoIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of one of the needle bytes in

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Two { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Send**
- **Copy**
- **Unpin**
#### Struct `TwoIter`

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Two::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct TwoIter<''a, ''h> {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TwoIter<''a, ''h> { /* ... */ }
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Three`

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

```rust
pub struct Three {
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
  pub fn new(needle1: u8, needle2: u8, needle3: u8) -> Three { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the three needle bytes

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of one of the needle bytes in the given

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of one of the needle bytes in the given

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> ThreeIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of one of the needle bytes in

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Three { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
#### Struct `ThreeIter`

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Three::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct ThreeIter<''a, ''h> {
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
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

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ThreeIter<''a, ''h> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
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

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `packedpair`

Provides an architecture independent implementation of the "packed pair"
algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for. Note that
this module provides an architecture independent version that doesn't do as
good of a job keeping the search for candidates inside a SIMD hot path. It
however can be good enough in many circumstances.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last

```rust
pub mod packedpair { /* ... */ }
```

### Types

#### Struct `Finder`

An architecture independent "packed pair" finder.

This finder picks two bytes that it believes have high predictive power for
indicating an overall match of a needle. At search time, it reports offsets
where the needle could match based on whether the pair of bytes it chose
match.

This is architecture independent because it utilizes `memchr` to find the
occurrence of one of the bytes in the pair, and then checks whether the
second byte matches. If it does, in the case of [`Finder::find_prefilter`],
the location at which the needle could match is returned.

It is generally preferred to use architecture specific routines for a
"packed pair" prefilter, but this can be a useful fallback when the
architecture independent routines are unavailable.

```rust
pub struct Finder {
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
  pub fn new(needle: &[u8]) -> Option<Finder> { /* ... */ }
  ```
  Create a new prefilter that reports possible locations where the given

- ```rust
  pub fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder> { /* ... */ }
  ```
  Create a new prefilter using the pair given.

- ```rust
  pub fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Run this finder on the given haystack as a prefilter.

- ```rust
  pub fn pair(self: &Self) -> &Pair { /* ... */ }
  ```
  Returns the pair of offsets (into the needle) used to check as a

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

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Finder { /* ... */ }
    ```

- **Unpin**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Pair`

A pair of byte offsets into a needle to use as a predicate.

This pair is used as a predicate to quickly filter out positions in a
haystack in which a needle cannot match. In some cases, this pair can even
be used in vector algorithms such that the vector algorithm only switches
over to scalar code once this pair has been found.

A pair of offsets can be used in both substring search implementations and
in prefilters. The former will report matches of a needle in a haystack
where as the latter will only report possible matches of a needle.

The offsets are limited each to a maximum of 255 to keep memory usage low.
Moreover, it's rarely advantageous to create a predicate using offsets
greater than 255 anyway.

The only guarantee enforced on the pair of offsets is that they are not
equivalent. It is not necessarily the case that `index1 < index2` for
example. By convention, `index1` corresponds to the byte in the needle
that is believed to be most the predictive. Note also that because of the
requirement that the indices be both valid for the needle used to build
the pair and not equal, it follows that a pair can only be constructed for
needles with length at least 2.

```rust
pub struct Pair {
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
  pub fn new(needle: &[u8]) -> Option<Pair> { /* ... */ }
  ```
  Create a new pair of offsets from the given needle.

- ```rust
  pub fn with_ranker<R: HeuristicFrequencyRank>(needle: &[u8], ranker: R) -> Option<Pair> { /* ... */ }
  ```
  Create a new pair of offsets from the given needle and ranker.

- ```rust
  pub fn with_indices(needle: &[u8], index1: u8, index2: u8) -> Option<Pair> { /* ... */ }
  ```
  Create a new pair using the offsets given for the needle given.

- ```rust
  pub fn index1(self: &Self) -> u8 { /* ... */ }
  ```
  Returns the first offset of the pair.

- ```rust
  pub fn index2(self: &Self) -> u8 { /* ... */ }
  ```
  Returns the second offset of the pair.

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pair { /* ... */ }
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

- **Sync**
- **Send**
### Traits

#### Trait `HeuristicFrequencyRank`

This trait allows the user to customize the heuristic used to determine the
relative frequency of a given byte in the dataset being searched.

The use of this trait can have a dramatic impact on performance depending
on the type of data being searched. The details of why are explained in the
docs of [`crate::memmem::Prefilter`]. To summarize, the core algorithm uses
a prefilter to quickly identify candidate matches that are later verified
more slowly. This prefilter is implemented in terms of trying to find
`rare` bytes at specific offsets that will occur less frequently in the
dataset. While the concept of a `rare` byte is similar for most datasets,
there are some specific datasets (like binary executables) that have
dramatically different byte distributions. For these datasets customizing
the byte frequency heuristic can have a massive impact on performance, and
might even need to be done at runtime.

The default implementation of `HeuristicFrequencyRank` reads from the
static frequency table defined in `src/memmem/byte_frequencies.rs`. This
is optimal for most inputs, so if you are unsure of the impact of using a
custom `HeuristicFrequencyRank` you should probably just use the default.

# Example

```
use memchr::{
    arch::all::packedpair::HeuristicFrequencyRank,
    memmem::FinderBuilder,
};

/// A byte-frequency table that is good for scanning binary executables.
struct Binary;

impl HeuristicFrequencyRank for Binary {
    fn rank(&self, byte: u8) -> u8 {
        const TABLE: [u8; 256] = [
            255, 128, 61, 43, 50, 41, 27, 28, 57, 15, 21, 13, 24, 17, 17,
            89, 58, 16, 11, 7, 14, 23, 7, 6, 24, 9, 6, 5, 9, 4, 7, 16,
            68, 11, 9, 6, 88, 7, 4, 4, 23, 9, 4, 8, 8, 5, 10, 4, 30, 11,
            9, 24, 11, 5, 5, 5, 19, 11, 6, 17, 9, 9, 6, 8,
            48, 58, 11, 14, 53, 40, 9, 9, 254, 35, 3, 6, 52, 23, 6, 6, 27,
            4, 7, 11, 14, 13, 10, 11, 11, 5, 2, 10, 16, 12, 6, 19,
            19, 20, 5, 14, 16, 31, 19, 7, 14, 20, 4, 4, 19, 8, 18, 20, 24,
            1, 25, 19, 58, 29, 10, 5, 15, 20, 2, 2, 9, 4, 3, 5,
            51, 11, 4, 53, 23, 39, 6, 4, 13, 81, 4, 186, 5, 67, 3, 2, 15,
            0, 0, 1, 3, 2, 0, 0, 5, 0, 0, 0, 2, 0, 0, 0,
            12, 2, 1, 1, 3, 1, 1, 1, 6, 1, 2, 1, 3, 1, 1, 2, 9, 1, 1, 0,
            2, 2, 4, 4, 11, 6, 7, 3, 6, 9, 4, 5,
            46, 18, 8, 18, 17, 3, 8, 20, 16, 10, 3, 7, 175, 4, 6, 7, 13,
            3, 7, 3, 3, 1, 3, 3, 10, 3, 1, 5, 2, 0, 1, 2,
            16, 3, 5, 1, 6, 1, 1, 2, 58, 20, 3, 14, 12, 2, 1, 3, 16, 3, 5,
            8, 3, 1, 8, 6, 17, 6, 5, 3, 8, 6, 13, 175,
        ];
        TABLE[byte as usize]
    }
}
// Create a new finder with the custom heuristic.
let finder = FinderBuilder::new()
    .build_forward_with_ranker(Binary, b"\x00\x00\xdd\xdd");
// Find needle with custom heuristic.
assert!(finder.find(b"\x00\x00\x00\xdd\xdd").is_some());
```

```rust
pub trait HeuristicFrequencyRank {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `rank`: Return the heuristic frequency rank of the given byte. A lower rank

##### Implementations

This trait is implemented for the following types:

- `&''a R` with <''a, R>

## Module `rabinkarp`

An implementation of the [Rabin-Karp substring search algorithm][rabinkarp].

Rabin-Karp works by creating a hash of the needle provided and then computing
a rolling hash for each needle sized window in the haystack. When the rolling
hash matches the hash of the needle, a byte-wise comparison is done to check
if a match exists. The worst case time complexity of Rabin-Karp is `O(m *
n)` where `m ~ len(needle)` and `n ~ len(haystack)`. Its worst case space
complexity is constant.

The main utility of Rabin-Karp is that the searcher can be constructed very
quickly with very little memory. This makes it especially useful when searching
for small needles in small haystacks, as it might finish its search before a
beefier algorithm (like Two-Way) even starts.

[rabinkarp]: https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm

```rust
pub mod rabinkarp { /* ... */ }
```

### Types

#### Struct `Finder`

A forward substring searcher using the Rabin-Karp algorithm.

Note that, as a lower level API, a `Finder` does not have access to the
needle it was constructed with. For this reason, executing a search
with a `Finder` requires passing both the needle and the haystack,
where the needle is exactly equivalent to the one given to the `Finder`
at construction time. This design was chosen so that callers can have
more precise control over where and how many times a needle is stored.
For example, in cases where Rabin-Karp is just one of several possible
substring search algorithms.

```rust
pub struct Finder {
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
  pub fn new(needle: &[u8]) -> Finder { /* ... */ }
  ```
  Create a new Rabin-Karp forward searcher for the given `needle`.

- ```rust
  pub fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of the `needle` in the `haystack`

- ```rust
  pub unsafe fn find_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Finder { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

#### Struct `FinderRev`

A reverse substring searcher using the Rabin-Karp algorithm.

```rust
pub struct FinderRev(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle: &[u8]) -> FinderRev { /* ... */ }
  ```
  Create a new Rabin-Karp reverse searcher for the given `needle`.

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of the `needle` in the `haystack`

- ```rust
  pub unsafe fn rfind_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FinderRev { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Module `shiftor`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An implementation of the [Shift-Or substring search algorithm][shiftor].

[shiftor]: https://en.wikipedia.org/wiki/Bitap_algorithm

```rust
pub mod shiftor { /* ... */ }
```

### Types

#### Struct `Finder`

A forward substring searcher using the Shift-Or algorithm.

```rust
pub struct Finder {
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
  pub fn new(needle: &[u8]) -> Option<Finder> { /* ... */ }
  ```
  Create a new Shift-Or forward searcher for the given `needle`.

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of the needle given to `Finder::new` in

###### Trait Implementations

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

- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
## Module `twoway`

An implementation of the [Two-Way substring search algorithm][two-way].

[`Finder`] can be built for forward searches, while [`FinderRev`] can be built
for reverse searches.

Two-Way makes for a nice general purpose substring search algorithm because of
its time and space complexity properties. It also performs well in practice.
Namely, with `m = len(needle)` and `n = len(haystack)`, Two-Way takes `O(m)`
time to create a finder, `O(1)` space and `O(n)` search time. In other words,
the preprocessing step is quick, doesn't require any heap memory and the worst
case search time is guaranteed to be linear in the haystack regardless of the
size of the needle.

While vector algorithms will usually beat Two-Way handedly, vector algorithms
also usually have pathological or edge cases that are better handled by Two-Way.
Moreover, not all targets support vector algorithms or implementations for them
simply may not exist yet.

Two-Way can be found in the `memmem` implementations in at least [GNU libc] and
[musl].

[two-way]: https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm
[GNU libc]: https://www.gnu.org/software/libc/
[musl]: https://www.musl-libc.org/

```rust
pub mod twoway { /* ... */ }
```

### Types

#### Struct `Finder`

A forward substring searcher that uses the Two-Way algorithm.

```rust
pub struct Finder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle: &[u8]) -> Finder { /* ... */ }
  ```
  Create a searcher that finds occurrences of the given `needle`.

- ```rust
  pub fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Returns the first occurrence of `needle` in the given `haystack`, or

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Finder { /* ... */ }
    ```

#### Struct `FinderRev`

A reverse substring searcher that uses the Two-Way algorithm.

```rust
pub struct FinderRev(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle: &[u8]) -> FinderRev { /* ... */ }
  ```
  Create a searcher that finds occurrences of the given `needle`.

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Returns the last occurrence of `needle` in the given `haystack`, or

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Unpin**
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

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FinderRev { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
### Functions

#### Function `is_prefix`

**Attributes:**

- `#[inline(always)]`

Returns true if and only if `needle` is a prefix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

```rust
pub fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool { /* ... */ }
```

#### Function `is_suffix`

**Attributes:**

- `#[inline(always)]`

Returns true if and only if `needle` is a suffix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

```rust
pub fn is_suffix(haystack: &[u8], needle: &[u8]) -> bool { /* ... */ }
```

#### Function `is_equal`

**Attributes:**

- `#[inline(always)]`

Compare corresponding bytes in `x` and `y` for equality.

That is, this returns true if and only if `x.len() == y.len()` and
`x[i] == y[i]` for all `0 <= i < x.len()`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

```rust
pub fn is_equal(x: &[u8], y: &[u8]) -> bool { /* ... */ }
```

#### Function `is_equal_raw`

**Attributes:**

- `#[inline(always)]`

Compare `n` bytes at the given pointers for equality.

This returns true if and only if `*x.add(i) == *y.add(i)` for all
`0 <= i < n`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

# Safety

* Both `x` and `y` must be valid for reads of up to `n` bytes.
* Both `x` and `y` must point to an initialized value.
* Both `x` and `y` must each point to an allocated object and
must either be in bounds or at most one byte past the end of the
allocated object. `x` and `y` do not need to point to the same allocated
object, but they may.
* Both `x` and `y` must be _derived from_ a pointer to their respective
allocated objects.
* The distance between `x` and `x+n` must not overflow `isize`. Similarly
for `y` and `y+n`.
* The distance being in bounds must not rely on "wrapping around" the
address space.

```rust
pub unsafe fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool { /* ... */ }
```

## Module `aarch64`

**Attributes:**

- `#[<cfg>(target_arch = "aarch64")]`

Vector algorithms for the `aarch64` target.

```rust
pub mod aarch64 { /* ... */ }
```

### Modules

## Module `neon`

Algorithms for the `aarch64` target using 128-bit vectors via NEON.

```rust
pub mod neon { /* ... */ }
```

### Modules

## Module `memchr`

This module defines 128-bit vector implementations of `memchr` and friends.

The main types in this module are [`One`], [`Two`] and [`Three`]. They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers are
typically much faster than scalar routines accomplishing the same task.

The `One` searcher also provides a [`One::count`] routine for efficiently
counting the number of times a single byte occurs in a haystack. This is
useful, for example, for counting the number of lines in a haystack. This
routine exists because it is usually faster, especially with a high match
count, then using [`One::find`] repeatedly. ([`OneIter`] specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.

```rust
pub mod memchr { /* ... */ }
```

### Types

#### Struct `One`

Finds all occurrences of a single byte in a haystack.

```rust
pub struct One(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle: u8) -> Option<One> { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the needle byte given.

- ```rust
  pub unsafe fn new_unchecked(needle: u8) -> One { /* ... */ }
  ```
  Create a new finder specific to neon vectors and routines without

- ```rust
  pub fn is_available() -> bool { /* ... */ }
  ```
  Returns true when this implementation is available in the current

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of one of the needle bytes in the given

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of one of the needle bytes in the given

- ```rust
  pub fn count(self: &Self, haystack: &[u8]) -> usize { /* ... */ }
  ```
  Counts all occurrences of this byte in the given haystack.

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize { /* ... */ }
  ```
  Like `count`, but accepts and returns raw pointers.

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> OneIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of the needle byte in the

###### Trait Implementations

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> One { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `OneIter`

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`One::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct OneIter<''a, ''h> {
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
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FusedIterator**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OneIter<''a, ''h> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Two`

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

```rust
pub struct Two(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle1: u8, needle2: u8) -> Option<Two> { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the needle bytes given.

- ```rust
  pub unsafe fn new_unchecked(needle1: u8, needle2: u8) -> Two { /* ... */ }
  ```
  Create a new finder specific to neon vectors and routines without

- ```rust
  pub fn is_available() -> bool { /* ... */ }
  ```
  Returns true when this implementation is available in the current

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of one of the needle bytes in the given

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of one of the needle bytes in the given

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> TwoIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of the needle bytes in the

###### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Two { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `TwoIter`

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Two::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct TwoIter<''a, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TwoIter<''a, ''h> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **FusedIterator**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
#### Struct `Three`

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

```rust
pub struct Three(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle1: u8, needle2: u8, needle3: u8) -> Option<Three> { /* ... */ }
  ```
  Create a new searcher that finds occurrences of the needle bytes given.

- ```rust
  pub unsafe fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three { /* ... */ }
  ```
  Create a new finder specific to neon vectors and routines without

- ```rust
  pub fn is_available() -> bool { /* ... */ }
  ```
  Returns true when this implementation is available in the current

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the first occurrence of one of the needle bytes in the given

- ```rust
  pub fn rfind(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Return the last occurrence of one of the needle bytes in the given

- ```rust
  pub unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `find`, but accepts and returns raw pointers.

- ```rust
  pub unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8> { /* ... */ }
  ```
  Like `rfind`, but accepts and returns raw pointers.

- ```rust
  pub fn iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> ThreeIter<''a, ''h> { /* ... */ }
  ```
  Returns an iterator over all occurrences of the needle byte in the

###### Trait Implementations

- **Copy**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Three { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ThreeIter`

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Three::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct ThreeIter<''a, ''h> {
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
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **FusedIterator**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ThreeIter<''a, ''h> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `packedpair`

A 128-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last

```rust
pub mod packedpair { /* ... */ }
```

### Types

#### Struct `Finder`

A "packed pair" finder that uses 128-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`] of bytes given matches.

```rust
pub struct Finder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(needle: &[u8]) -> Option<Finder> { /* ... */ }
  ```
  Create a new pair searcher. The searcher returned can either report

- ```rust
  pub fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder> { /* ... */ }
  ```
  Create a new "packed pair" finder using the pair of bytes given.

- ```rust
  pub fn is_available() -> bool { /* ... */ }
  ```
  Returns true when this implementation is available in the current

- ```rust
  pub fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Execute a search using neon vectors and routines.

- ```rust
  pub fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Execute a search using neon vectors and routines.

- ```rust
  pub fn pair(self: &Self) -> &Pair { /* ... */ }
  ```
  Returns the pair of offsets (into the needle) used to check as a

- ```rust
  pub fn min_haystack_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the minimum haystack length that this `Finder` can search.

###### Trait Implementations

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Finder { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
## Module `memmem`

This module provides forward and reverse substring search routines.

Unlike the standard library's substring search routines, these work on
arbitrary bytes. For all non-empty needles, these routines will report exactly
the same values as the corresponding routines in the standard library. For
the empty needle, the standard library reports matches only at valid UTF-8
boundaries, where as these routines will report matches at every position.

Other than being able to work on arbitrary bytes, the primary reason to prefer
these routines over the standard library routines is that these will generally
be faster. In some cases, significantly so.

# Example: iterating over substring matches

This example shows how to use [`find_iter`] to find occurrences of a substring
in a haystack.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches in reverse

This example shows how to use [`rfind_iter`] to find occurrences of a substring
in a haystack starting from the end of the haystack.

**NOTE:** This module does not implement double ended iterators, so reverse
searches aren't done by calling `rev` on a forward iterator.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::rfind_iter(haystack, "foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

# Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher to be
measurable in some workloads. In cases where the same needle is used to search
many haystacks, it is possible to do construction once and thus to avoid it for
subsequent searches. This can be done with a [`Finder`] (or a [`FinderRev`] for
reverse searches).

```
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

```rust
pub mod memmem { /* ... */ }
```

### Types

#### Struct `FindIter`

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

```rust
pub struct FindIter<''h, ''n> {
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
  pub fn into_owned(self: Self) -> FindIter<''h, ''static> { /* ... */ }
  ```
  Convert this iterator into its owned variant, such that it no longer

###### Trait Implementations

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FindIter<''h, ''n> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `FindRevIter`

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

```rust
pub struct FindRevIter<''h, ''n> {
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
  pub fn into_owned(self: Self) -> FindRevIter<''h, ''static> { /* ... */ }
  ```
  Convert this iterator into its owned variant, such that it no longer

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FindRevIter<''h, ''n> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn next(self: &mut Self) -> Option<usize> { /* ... */ }
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

- **RefUnwindSafe**
- **Sync**
- **Freeze**
#### Struct `Finder`

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`find`] is good enough, but `Finder` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to
the lifetime of its needle.

```rust
pub struct Finder<''n> {
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
  pub fn new<B: ?Sized + AsRef<[u8]>>(needle: &''n B) -> Finder<''n> { /* ... */ }
  ```
  Create a new finder for the given needle.

- ```rust
  pub fn find(self: &Self, haystack: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Returns the index of the first occurrence of this needle in the given

- ```rust
  pub fn find_iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> FindIter<''h, ''a> { /* ... */ }
  ```
  Returns an iterator over all occurrences of a substring in a haystack.

- ```rust
  pub fn into_owned(self: Self) -> Finder<''static> { /* ... */ }
  ```
  Convert this finder into its owned variant, such that it no longer

- ```rust
  pub fn as_ref(self: &Self) -> Finder<''_> { /* ... */ }
  ```
  Convert this finder into its borrowed variant.

- ```rust
  pub fn needle(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the needle that this finder searches for.

###### Trait Implementations

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Finder<''n> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

#### Struct `FinderRev`

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general,
using [`rfind`] is good enough, but `FinderRev` is useful when you can
meaningfully observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderRev` that is not connected to
the lifetime of its needle.

```rust
pub struct FinderRev<''n> {
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
  pub fn new<B: ?Sized + AsRef<[u8]>>(needle: &''n B) -> FinderRev<''n> { /* ... */ }
  ```
  Create a new reverse finder for the given needle.

- ```rust
  pub fn rfind<B: AsRef<[u8]>>(self: &Self, haystack: B) -> Option<usize> { /* ... */ }
  ```
  Returns the index of the last occurrence of this needle in the given

- ```rust
  pub fn rfind_iter<''a, ''h>(self: &''a Self, haystack: &''h [u8]) -> FindRevIter<''h, ''a> { /* ... */ }
  ```
  Returns a reverse iterator over all occurrences of a substring in a

- ```rust
  pub fn into_owned(self: Self) -> FinderRev<''static> { /* ... */ }
  ```
  Convert this finder into its owned variant, such that it no longer

- ```rust
  pub fn as_ref(self: &Self) -> FinderRev<''_> { /* ... */ }
  ```
  Convert this finder into its borrowed variant.

- ```rust
  pub fn needle(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the needle that this finder searches for.

###### Trait Implementations

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FinderRev<''n> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `FinderBuilder`

A builder for constructing non-default forward or reverse memmem finders.

A builder is primarily useful for configuring a substring searcher.
Currently, the only configuration exposed is the ability to disable
heuristic prefilters used to speed up certain searches.

```rust
pub struct FinderBuilder {
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
  pub fn new() -> FinderBuilder { /* ... */ }
  ```
  Create a new finder builder with default settings.

- ```rust
  pub fn build_forward<''n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &''n B) -> Finder<''n> { /* ... */ }
  ```
  Build a forward finder using the given needle from the current

- ```rust
  pub fn build_forward_with_ranker<''n, R: HeuristicFrequencyRank, B: ?Sized + AsRef<[u8]>>(self: &Self, ranker: R, needle: &''n B) -> Finder<''n> { /* ... */ }
  ```
  Build a forward finder using the given needle and a custom heuristic for

- ```rust
  pub fn build_reverse<''n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &''n B) -> FinderRev<''n> { /* ... */ }
  ```
  Build a reverse finder using the given needle from the current

- ```rust
  pub fn prefilter(self: &mut Self, prefilter: Prefilter) -> &mut FinderBuilder { /* ... */ }
  ```
  Configure the prefilter setting for the finder.

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
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

- **Default**
  - ```rust
    fn default() -> FinderBuilder { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FinderBuilder { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Unpin**
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `find_iter`

**Attributes:**

- `#[inline]`

Returns an iterator over all non-overlapping occurrences of a substring in
a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::find_iter(haystack, b"foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

```rust
pub fn find_iter<''h, ''n, N: ''n + ?Sized + AsRef<[u8]>>(haystack: &''h [u8], needle: &''n N) -> FindIter<''h, ''n> { /* ... */ }
```

#### Function `rfind_iter`

**Attributes:**

- `#[inline]`

Returns a reverse iterator over all non-overlapping occurrences of a
substring in a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::rfind_iter(haystack, b"foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

```rust
pub fn rfind_iter<''h, ''n, N: ''n + ?Sized + AsRef<[u8]>>(haystack: &''h [u8], needle: &''n N) -> FindRevIter<''h, ''n> { /* ... */ }
```

#### Function `find`

**Attributes:**

- `#[inline]`

Returns the index of the first occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`Finder`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::find(haystack, b"foo"));
assert_eq!(Some(4), memmem::find(haystack, b"bar"));
assert_eq!(None, memmem::find(haystack, b"quux"));
```

```rust
pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
```

#### Function `rfind`

**Attributes:**

- `#[inline]`

Returns the index of the last occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`FinderRev`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::rfind(haystack, b"foo"));
assert_eq!(Some(4), memmem::rfind(haystack, b"bar"));
assert_eq!(Some(8), memmem::rfind(haystack, b"ba"));
assert_eq!(None, memmem::rfind(haystack, b"quux"));
```

```rust
pub fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize> { /* ... */ }
```

### Re-exports

#### Re-export `PrefilterConfig`

```rust
pub use crate::memmem::searcher::PrefilterConfig as Prefilter;
```

## Re-exports

### Re-export `memchr`

```rust
pub use crate::memchr::memchr;
```

### Re-export `memchr2`

```rust
pub use crate::memchr::memchr2;
```

### Re-export `memchr2_iter`

```rust
pub use crate::memchr::memchr2_iter;
```

### Re-export `memchr3`

```rust
pub use crate::memchr::memchr3;
```

### Re-export `memchr3_iter`

```rust
pub use crate::memchr::memchr3_iter;
```

### Re-export `memchr_iter`

```rust
pub use crate::memchr::memchr_iter;
```

### Re-export `memrchr`

```rust
pub use crate::memchr::memrchr;
```

### Re-export `memrchr2`

```rust
pub use crate::memchr::memrchr2;
```

### Re-export `memrchr2_iter`

```rust
pub use crate::memchr::memrchr2_iter;
```

### Re-export `memrchr3`

```rust
pub use crate::memchr::memrchr3;
```

### Re-export `memrchr3_iter`

```rust
pub use crate::memchr::memrchr3_iter;
```

### Re-export `memrchr_iter`

```rust
pub use crate::memchr::memrchr_iter;
```

### Re-export `Memchr`

```rust
pub use crate::memchr::Memchr;
```

### Re-export `Memchr2`

```rust
pub use crate::memchr::Memchr2;
```

### Re-export `Memchr3`

```rust
pub use crate::memchr::Memchr3;
```

