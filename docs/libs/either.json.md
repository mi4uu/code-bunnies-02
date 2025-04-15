# Crate Documentation

**Version:** 1.15.0

**Format Version:** 43

# Module `either`

The enum [`Either`] with variants `Left` and `Right` is a general purpose
sum type with two cases.

[`Either`]: enum.Either.html

**Crate features:**

* `"std"`
  Enabled by default. Disable to make the library `#![no_std]`.

* `"serde"`
  Disabled by default. Enable to `#[derive(Serialize, Deserialize)]` for `Either`


## Types

### Enum `Either`

**Attributes:**

- `#[<cfg_attr>(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`

The enum `Either` with variants `Left` and `Right` is a general purpose
sum type with two cases.

The `Either` type is symmetric and treats its variants the same way, without
preference.
(For representing success or error, use the regular `Result` enum instead.)

```rust
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
```

#### Variants

##### `Left`

A value of type `L`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `L` |  |

##### `Right`

A value of type `R`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `R` |  |

#### Implementations

##### Methods

- ```rust
  pub fn is_left(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the value is the `Left` variant.

- ```rust
  pub fn is_right(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the value is the `Right` variant.

- ```rust
  pub fn left(self: Self) -> Option<L> { /* ... */ }
  ```
  Convert the left side of `Either<L, R>` to an `Option<L>`.

- ```rust
  pub fn right(self: Self) -> Option<R> { /* ... */ }
  ```
  Convert the right side of `Either<L, R>` to an `Option<R>`.

- ```rust
  pub fn as_ref(self: &Self) -> Either<&L, &R> { /* ... */ }
  ```
  Convert `&Either<L, R>` to `Either<&L, &R>`.

- ```rust
  pub fn as_mut(self: &mut Self) -> Either<&mut L, &mut R> { /* ... */ }
  ```
  Convert `&mut Either<L, R>` to `Either<&mut L, &mut R>`.

- ```rust
  pub fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&L>, Pin<&R>> { /* ... */ }
  ```
  Convert `Pin<&Either<L, R>>` to `Either<Pin<&L>, Pin<&R>>`,

- ```rust
  pub fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut L>, Pin<&mut R>> { /* ... */ }
  ```
  Convert `Pin<&mut Either<L, R>>` to `Either<Pin<&mut L>, Pin<&mut R>>`,

- ```rust
  pub fn flip(self: Self) -> Either<R, L> { /* ... */ }
  ```
  Convert `Either<L, R>` to `Either<R, L>`.

- ```rust
  pub fn map_left<F, M>(self: Self, f: F) -> Either<M, R>
where
    F: FnOnce(L) -> M { /* ... */ }
  ```
  Apply the function `f` on the value in the `Left` variant if it is present rewrapping the

- ```rust
  pub fn map_right<F, S>(self: Self, f: F) -> Either<L, S>
where
    F: FnOnce(R) -> S { /* ... */ }
  ```
  Apply the function `f` on the value in the `Right` variant if it is present rewrapping the

- ```rust
  pub fn map_either<F, G, M, S>(self: Self, f: F, g: G) -> Either<M, S>
where
    F: FnOnce(L) -> M,
    G: FnOnce(R) -> S { /* ... */ }
  ```
  Apply the functions `f` and `g` to the `Left` and `Right` variants

- ```rust
  pub fn map_either_with<Ctx, F, G, M, S>(self: Self, ctx: Ctx, f: F, g: G) -> Either<M, S>
where
    F: FnOnce(Ctx, L) -> M,
    G: FnOnce(Ctx, R) -> S { /* ... */ }
  ```
  Similar to [`map_either`][Self::map_either], with an added context `ctx` accessible to

- ```rust
  pub fn either<F, G, T>(self: Self, f: F, g: G) -> T
where
    F: FnOnce(L) -> T,
    G: FnOnce(R) -> T { /* ... */ }
  ```
  Apply one of two functions depending on contents, unifying their result. If the value is

- ```rust
  pub fn either_with<Ctx, F, G, T>(self: Self, ctx: Ctx, f: F, g: G) -> T
where
    F: FnOnce(Ctx, L) -> T,
    G: FnOnce(Ctx, R) -> T { /* ... */ }
  ```
  Like [`either`][Self::either], but provide some context to whichever of the

- ```rust
  pub fn left_and_then<F, S>(self: Self, f: F) -> Either<S, R>
where
    F: FnOnce(L) -> Either<S, R> { /* ... */ }
  ```
  Apply the function `f` on the value in the `Left` variant if it is present.

- ```rust
  pub fn right_and_then<F, S>(self: Self, f: F) -> Either<L, S>
where
    F: FnOnce(R) -> Either<L, S> { /* ... */ }
  ```
  Apply the function `f` on the value in the `Right` variant if it is present.

- ```rust
  pub fn into_iter(self: Self) -> Either<<L as >::IntoIter, <R as >::IntoIter>
where
    L: IntoIterator,
    R: IntoIterator<Item = <L as >::Item> { /* ... */ }
  ```
  Convert the inner value to an iterator.

- ```rust
  pub fn iter(self: &Self) -> Either<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
where
    for<''a> &''a L: IntoIterator,
    for<''a> &''a R: IntoIterator<Item = <&''a L as IntoIterator>::Item> { /* ... */ }
  ```
  Borrow the inner value as an iterator.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Either<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>
where
    for<''a> &''a mut L: IntoIterator,
    for<''a> &''a mut R: IntoIterator<Item = <&''a mut L as IntoIterator>::Item> { /* ... */ }
  ```
  Mutably borrow the inner value as an iterator.

- ```rust
  pub fn factor_into_iter(self: Self) -> IterEither<<L as >::IntoIter, <R as >::IntoIter>
where
    L: IntoIterator,
    R: IntoIterator { /* ... */ }
  ```
  Converts an `Either` of `Iterator`s to be an `Iterator` of `Either`s

- ```rust
  pub fn factor_iter(self: &Self) -> IterEither<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
where
    for<''a> &''a L: IntoIterator,
    for<''a> &''a R: IntoIterator { /* ... */ }
  ```
  Borrows an `Either` of `Iterator`s to be an `Iterator` of `Either`s

- ```rust
  pub fn factor_iter_mut(self: &mut Self) -> IterEither<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>
where
    for<''a> &''a mut L: IntoIterator,
    for<''a> &''a mut R: IntoIterator { /* ... */ }
  ```
  Mutably borrows an `Either` of `Iterator`s to be an `Iterator` of `Either`s

- ```rust
  pub fn left_or(self: Self, other: L) -> L { /* ... */ }
  ```
  Return left value or given value

- ```rust
  pub fn left_or_default(self: Self) -> L
where
    L: Default { /* ... */ }
  ```
  Return left or a default

- ```rust
  pub fn left_or_else<F>(self: Self, f: F) -> L
where
    F: FnOnce(R) -> L { /* ... */ }
  ```
  Returns left value or computes it from a closure

- ```rust
  pub fn right_or(self: Self, other: R) -> R { /* ... */ }
  ```
  Return right value or given value

- ```rust
  pub fn right_or_default(self: Self) -> R
where
    R: Default { /* ... */ }
  ```
  Return right or a default

- ```rust
  pub fn right_or_else<F>(self: Self, f: F) -> R
where
    F: FnOnce(L) -> R { /* ... */ }
  ```
  Returns right value or computes it from a closure

- ```rust
  pub fn unwrap_left(self: Self) -> L
where
    R: core::fmt::Debug { /* ... */ }
  ```
  Returns the left value

- ```rust
  pub fn unwrap_right(self: Self) -> R
where
    L: core::fmt::Debug { /* ... */ }
  ```
  Returns the right value

- ```rust
  pub fn expect_left(self: Self, msg: &str) -> L
where
    R: core::fmt::Debug { /* ... */ }
  ```
  Returns the left value

- ```rust
  pub fn expect_right(self: Self, msg: &str) -> R
where
    L: core::fmt::Debug { /* ... */ }
  ```
  Returns the right value

- ```rust
  pub fn either_into<T>(self: Self) -> T
where
    L: Into<T>,
    R: Into<T> { /* ... */ }
  ```
  Convert the contained value into `T`

- ```rust
  pub fn factor_none(self: Self) -> Option<Either<L, R>> { /* ... */ }
  ```
  Factors out `None` from an `Either` of [`Option`].

- ```rust
  pub fn factor_err(self: Self) -> Result<Either<L, R>, E> { /* ... */ }
  ```
  Factors out a homogenous type from an `Either` of [`Result`].

- ```rust
  pub fn factor_ok(self: Self) -> Result<T, Either<L, R>> { /* ... */ }
  ```
  Factors out a homogenous type from an `Either` of [`Result`].

- ```rust
  pub fn factor_first(self: Self) -> (T, Either<L, R>) { /* ... */ }
  ```
  Factor out a homogeneous type from an either of pairs.

- ```rust
  pub fn factor_second(self: Self) -> (Either<L, R>, T) { /* ... */ }
  ```
  Factor out a homogeneous type from an either of pairs.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Extract the value of an either over two equivalent types.

- ```rust
  pub fn map<F, M>(self: Self, f: F) -> Either<M, M>
where
    F: FnOnce(T) -> M { /* ... */ }
  ```
  Map `f` over the contained value and return the result in the

- ```rust
  pub fn cloned(self: Self) -> Either<L, R>
where
    L: Clone,
    R: Clone { /* ... */ }
  ```
  Maps an `Either<&L, &R>` to an `Either<L, R>` by cloning the contents of

- ```rust
  pub fn copied(self: Self) -> Either<L, R>
where
    L: Copy,
    R: Copy { /* ... */ }
  ```
  Maps an `Either<&L, &R>` to an `Either<L, R>` by copying the contents of

- ```rust
  pub fn cloned(self: Self) -> Either<L, R>
where
    L: Clone,
    R: Clone { /* ... */ }
  ```
  Maps an `Either<&mut L, &mut R>` to an `Either<L, R>` by cloning the contents of

- ```rust
  pub fn copied(self: Self) -> Either<L, R>
where
    L: Copy,
    R: Copy { /* ... */ }
  ```
  Maps an `Either<&mut L, &mut R>` to an `Either<L, R>` by copying the contents of

##### Trait Implementations

- **Extend**
  - ```rust
    fn extend<T>(self: &mut Self, iter: T)
where
    T: IntoIterator<Item = A> { /* ... */ }
    ```

- **IntoEither**
- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Either<L, R>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn read_exact(self: &mut Self, buf: &mut [u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn read_to_end(self: &mut Self, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn read_to_string(self: &mut Self, buf: &mut std::string::String) -> io::Result<usize> { /* ... */ }
    ```

- **Seek**
  - ```rust
    fn seek(self: &mut Self, pos: SeekFrom) -> io::Result<u64> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Either<L, R>) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Either<L, R>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BufRead**
  - ```rust
    fn fill_buf(self: &mut Self) -> io::Result<&[u8]> { /* ... */ }
    ```

  - ```rust
    fn consume(self: &mut Self, amt: usize) { /* ... */ }
    ```

  - ```rust
    fn read_until(self: &mut Self, byte: u8, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn read_line(self: &mut Self, buf: &mut std::string::String) -> io::Result<usize> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, fmt: fmt::Arguments<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_str(self: &mut Self, s: &str) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn write_char(self: &mut Self, c: char) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, args: fmt::Arguments<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn rfold<Acc, G>(self: Self, init: Acc, f: G) -> Acc
where
    G: FnMut(Acc, <Self as >::Item) -> Acc { /* ... */ }
    ```

  - ```rust
    fn rfind<P>(self: &mut Self, predicate: P) -> Option<<Self as >::Item>
where
    P: FnMut(&<Self as >::Item) -> bool { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FusedIterator**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Target { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &::std::path::Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &::std::ffi::OsStr { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &::std::ffi::CStr { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[Target] { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut str { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut ::std::path::Path { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut ::std::ffi::OsStr { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut ::std::ffi::CStr { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut Target { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [Target] { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn fold<Acc, G>(self: Self, init: Acc, f: G) -> Acc
where
    G: FnMut(Acc, <Self as >::Item) -> Acc { /* ... */ }
    ```

  - ```rust
    fn for_each<F>(self: Self, f: F)
where
    F: FnMut(<Self as >::Item) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn collect<B>(self: Self) -> B
where
    B: iter::FromIterator<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn partition<B, F>(self: Self, f: F) -> (B, B)
where
    B: Default + Extend<<Self as >::Item>,
    F: FnMut(&<Self as >::Item) -> bool { /* ... */ }
    ```

  - ```rust
    fn all<F>(self: &mut Self, f: F) -> bool
where
    F: FnMut(<Self as >::Item) -> bool { /* ... */ }
    ```

  - ```rust
    fn any<F>(self: &mut Self, f: F) -> bool
where
    F: FnMut(<Self as >::Item) -> bool { /* ... */ }
    ```

  - ```rust
    fn find<P>(self: &mut Self, predicate: P) -> Option<<Self as >::Item>
where
    P: FnMut(&<Self as >::Item) -> bool { /* ... */ }
    ```

  - ```rust
    fn find_map<B, F>(self: &mut Self, f: F) -> Option<B>
where
    F: FnMut(<Self as >::Item) -> Option<B> { /* ... */ }
    ```

  - ```rust
    fn position<P>(self: &mut Self, predicate: P) -> Option<usize>
where
    P: FnMut(<Self as >::Item) -> bool { /* ... */ }
    ```

- **Unpin**
- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn Error + ''static> { /* ... */ }
    ```

  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn Error> { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<''_>) -> core::task::Poll<<Self as >::Output> { /* ... */ }
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

  - ```rust
    fn from(r: Result<R, L>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: Either<L, R>) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

## Macros

### Macro `for_both`

**Attributes:**

- `#[macro_export]`

Evaluate the provided expression for both [`Either::Left`] and [`Either::Right`].

This macro is useful in cases where both sides of [`Either`] can be interacted with
in the same way even though the don't share the same type.

Syntax: `either::for_both!(` *expression* `,` *pattern* `=>` *expression* `)`

# Example

```
use either::Either;

fn length(owned_or_borrowed: Either<String, &'static str>) -> usize {
    either::for_both!(owned_or_borrowed, s => s.len())
}

fn main() {
    let borrowed = Either::Right("Hello world!");
    let owned = Either::Left("Hello world!".to_owned());

    assert_eq!(length(borrowed), 12);
    assert_eq!(length(owned), 12);
}
```

```rust
pub macro_rules! for_both {
    /* macro_rules! for_both {
    ($value:expr, $pattern:pat => $result:expr) => { ... };
} */
}
```

### Macro `try_left`

**Attributes:**

- `#[macro_export]`

Macro for unwrapping the left side of an [`Either`], which fails early
with the opposite side. Can only be used in functions that return
`Either` because of the early return of `Right` that it provides.

See also [`try_right!`] for its dual, which applies the same just to the
right side.

# Example

```
use either::{Either, Left, Right};

fn twice(wrapper: Either<u32, &str>) -> Either<u32, &str> {
    let value = either::try_left!(wrapper);
    Left(value * 2)
}

fn main() {
    assert_eq!(twice(Left(2)), Left(4));
    assert_eq!(twice(Right("ups")), Right("ups"));
}
```

```rust
pub macro_rules! try_left {
    /* macro_rules! try_left {
    ($expr:expr) => { ... };
} */
}
```

### Macro `try_right`

**Attributes:**

- `#[macro_export]`

Dual to [`try_left!`], see its documentation for more information.

```rust
pub macro_rules! try_right {
    /* macro_rules! try_right {
    ($expr:expr) => { ... };
} */
}
```

## Re-exports

### Re-export `Left`

```rust
pub use crate::Either::Left;
```

### Re-export `Right`

```rust
pub use crate::Either::Right;
```

### Re-export `IterEither`

```rust
pub use self::iterator::IterEither;
```

### Re-export `IntoEither`

```rust
pub use self::into_either::IntoEither;
```

