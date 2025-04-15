# Crate Documentation

**Version:** 2.8.1

**Format Version:** 43

# Module `unicase`

# UniCase

UniCase provides a way of specifying strings that are case-insensitive.

UniCase supports full [Unicode case
folding](https://www.w3.org/International/wiki/Case_folding). It can also
utilize faster ASCII case comparisons, if both strings are ASCII.

Using the `UniCase::new()` constructor will check the string to see if it
is all ASCII. When a `UniCase` is compared against another, if both are
ASCII, it will use the faster comparison.

There also exists the `Ascii` type in this crate, which will always assume
to use the ASCII case comparisons, if the encoding is already known.

## Example

```rust
use unicase::UniCase;

let a = UniCase::new("Maße");
let b = UniCase::new("MASSE");
let c = UniCase::new("mase");

assert_eq!(a, b);
assert!(b != c);
```

## Ascii

```rust
use unicase::Ascii;

let a = Ascii::new("foobar");
let b = Ascii::new("FoObAr");

assert_eq!(a, b);
```

## Types

### Struct `UniCase`

Case Insensitive wrapper of strings.

```rust
pub struct UniCase<S>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(s: S) -> UniCase<S> { /* ... */ }
  ```
  Creates a new `UniCase`.

- ```rust
  pub fn to_folded_case(self: &Self) -> String { /* ... */ }
  ```
  Returns a copy of this string where each character is mapped to its

- ```rust
  pub const fn unicode(s: S) -> UniCase<S> { /* ... */ }
  ```
  Creates a new `UniCase`, skipping the ASCII check.

- ```rust
  pub const fn ascii(s: S) -> UniCase<S> { /* ... */ }
  ```
  Creates a new `UniCase` which performs only ASCII case folding.

- ```rust
  pub fn is_ascii(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this instance will only perform ASCII case folding.

- ```rust
  pub fn into_inner(self: Self) -> S { /* ... */ }
  ```
  Unwraps the inner value held by this `UniCase`.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniCase<S> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> &''a str { /* ... */ }
    ```

  - ```rust
    fn into(self: Self) -> String { /* ... */ }
    ```

  - ```rust
    fn into(self: Self) -> Cow<''a, str> { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<UniCase<S>, <Self as >::Err> { /* ... */ }
    ```

- **Freeze**
- **Receiver**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut<''a>(self: &''a mut Self) -> &''a mut S { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UniCase<S2>) -> bool { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref<''a>(self: &''a Self) -> &''a S { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(ascii: Ascii<S>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: S) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: &''a str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: &''a str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: Cow<''a, str>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: &''a String) -> Self { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Ascii`

Case Insensitive wrapper of Ascii strings.

```rust
pub struct Ascii<S>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(s: S) -> Ascii<S> { /* ... */ }
  ```
  Construct a new `Ascii`.

- ```rust
  pub const fn into_unicase(self: Self) -> UniCase<S> { /* ... */ }
  ```
  Convert this into a [`UniCase`].

- ```rust
  pub fn into_inner(self: Self) -> S { /* ... */ }
  ```
  Consume this `Ascii` and get the inner value.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(ascii: Ascii<S>) -> Self { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ascii<S1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Ascii<S1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &S2) -> bool { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Ascii<S>, <S as FromStr>::Err> { /* ... */ }
    ```

- **Receiver**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut<''a>(self: &''a mut Self) -> &''a mut S { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Ascii<S> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ascii<S> { /* ... */ }
    ```

- **Copy**
- **Deref**
  - ```rust
    fn deref<''a>(self: &''a Self) -> &''a S { /* ... */ }
    ```

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

## Functions

### Function `eq`

**Attributes:**

- `#[inline]`

Compare two string-like types for case-less equality, using unicode folding.

Equivalent to `UniCase::new(left) == UniCase::new(right)`.

Note: This will perform a scan for ASCII characters before doing the
the comparison. See `UniCase` for more information.

```rust
pub fn eq<S: AsRef<str> + ?Sized>(left: &S, right: &S) -> bool { /* ... */ }
```

### Function `eq_ascii`

**Attributes:**

- `#[inline]`

Compare two string-like types for case-less equality, ignoring ASCII case.

Equivalent to `Ascii::new(left) == Ascii::new(right)`.

```rust
pub fn eq_ascii<S: AsRef<str> + ?Sized>(left: &S, right: &S) -> bool { /* ... */ }
```

