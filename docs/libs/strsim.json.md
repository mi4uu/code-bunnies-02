# Crate Documentation

**Version:** 0.11.1

**Format Version:** 43

# Module `strsim`

This library implements string similarity metrics.

## Types

### Enum `StrSimError`

```rust
pub enum StrSimError {
    DifferentLengthArgs,
}
```

#### Variants

##### `DifferentLengthArgs`

#### Implementations

##### Trait Implementations

- **Error**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StrSimError) -> bool { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Type Alias `HammingResult`

```rust
pub type HammingResult = Result<usize, StrSimError>;
```

## Functions

### Function `generic_hamming`

Calculates the number of positions in the two sequences where the elements
differ. Returns an error if the sequences have different lengths.

```rust
pub fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2> { /* ... */ }
```

### Function `hamming`

Calculates the number of positions in the two strings where the characters
differ. Returns an error if the strings have different lengths.

```
use strsim::{hamming, StrSimError::DifferentLengthArgs};

assert_eq!(Ok(3), hamming("hamming", "hammers"));

assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
```

```rust
pub fn hamming(a: &str, b: &str) -> HammingResult { /* ... */ }
```

### Function `generic_jaro`

Calculates the Jaro similarity between two sequences. The returned value
is between 0.0 and 1.0 (higher value means more similar).

```rust
pub fn generic_jaro<''a, ''b, Iter1, Iter2, Elem1, Elem2>(a: &''a Iter1, b: &''b Iter2) -> f64
where
    &''a Iter1: IntoIterator<Item = Elem1>,
    &''b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2> { /* ... */ }
```

### Function `jaro`

Calculates the Jaro similarity between two strings. The returned value
is between 0.0 and 1.0 (higher value means more similar).

```
use strsim::jaro;

assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
        0.001);
```

```rust
pub fn jaro(a: &str, b: &str) -> f64 { /* ... */ }
```

### Function `generic_jaro_winkler`

Like Jaro but gives a boost to sequences that have a common prefix.

```rust
pub fn generic_jaro_winkler<''a, ''b, Iter1, Iter2, Elem1, Elem2>(a: &''a Iter1, b: &''b Iter2) -> f64
where
    &''a Iter1: IntoIterator<Item = Elem1>,
    &''b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2> { /* ... */ }
```

### Function `jaro_winkler`

Like Jaro but gives a boost to strings that have a common prefix.

```
use strsim::jaro_winkler;

assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
        0.001);
```

```rust
pub fn jaro_winkler(a: &str, b: &str) -> f64 { /* ... */ }
```

### Function `generic_levenshtein`

Calculates the minimum number of insertions, deletions, and substitutions
required to change one sequence into the other.

```
use strsim::generic_levenshtein;

assert_eq!(3, generic_levenshtein(&[1,2,3], &[1,2,3,4,5,6]));
```

```rust
pub fn generic_levenshtein<''a, ''b, Iter1, Iter2, Elem1, Elem2>(a: &''a Iter1, b: &''b Iter2) -> usize
where
    &''a Iter1: IntoIterator<Item = Elem1>,
    &''b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2> { /* ... */ }
```

### Function `levenshtein`

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```
use strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```

```rust
pub fn levenshtein(a: &str, b: &str) -> usize { /* ... */ }
```

### Function `normalized_levenshtein`

Calculates a normalized score of the Levenshtein algorithm between 0.0 and
1.0 (inclusive), where 1.0 means the strings are the same.

```
use strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_levenshtein("", "second").abs() < 0.00001);
assert!(normalized_levenshtein("first", "").abs() < 0.00001);
assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
```

```rust
pub fn normalized_levenshtein(a: &str, b: &str) -> f64 { /* ... */ }
```

### Function `osa_distance`

Like Levenshtein but allows for adjacent transpositions. Each substring can
only be edited once.

```
use strsim::osa_distance;

assert_eq!(3, osa_distance("ab", "bca"));
```

```rust
pub fn osa_distance(a: &str, b: &str) -> usize { /* ... */ }
```

### Function `generic_damerau_levenshtein`

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```
use strsim::generic_damerau_levenshtein;

assert_eq!(2, generic_damerau_levenshtein(&[1,2], &[2,3,1]));
```

```rust
pub fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone { /* ... */ }
```

### Function `damerau_levenshtein`

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```
use strsim::damerau_levenshtein;

assert_eq!(2, damerau_levenshtein("ab", "bca"));
```

```rust
pub fn damerau_levenshtein(a: &str, b: &str) -> usize { /* ... */ }
```

### Function `normalized_damerau_levenshtein`

Calculates a normalized score of the Damerau–Levenshtein algorithm between
0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.

```
use strsim::normalized_damerau_levenshtein;

assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
```

```rust
pub fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64 { /* ... */ }
```

### Function `sorensen_dice`

Calculates a Sørensen-Dice similarity distance using bigrams.
See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.

```
use strsim::sorensen_dice;

assert_eq!(1.0, sorensen_dice("", ""));
assert_eq!(0.0, sorensen_dice("", "a"));
assert_eq!(0.0, sorensen_dice("french", "quebec"));
assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
```

```rust
pub fn sorensen_dice(a: &str, b: &str) -> f64 { /* ... */ }
```

