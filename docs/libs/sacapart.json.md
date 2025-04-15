# Crate Documentation

**Version:** 2.0.0

**Format Version:** 43

# Module `sacapart`

## Types

### Struct `PartitionedSuffixArray`

A partitioned suffix array, that is faster to construct but finds
slightly worse matches in a slightly longer amount of time.

Suffix sorting is an expensive operation that is hard to parallelize
well. The idea behind a partitioned suffix array is to suffix sort
multiple parts of a text (in parallel) rather than the full text.

Using two partitions will result in *roughly* 2x faster construction
(assuming there are two cores available), but search will now take
O(2 * log n), and matches across the boundaries may be much worse.

For example, the text "totor" may be partitioned into "tot" and "or".
Looking for matches for "tor" may well only return a substring of "to",
at offset 0, because the first partition only has the suffixes "t",
"to", and "tot". The second partition only has the suffixes "or" and "r".
So, it finds the substring "(to)tor", tries to extend it to the right,
and fails. It doesn't try to extend "to(t)or", because in the suffix
array of that partition, that substring is a weaker match than "(to)tor".

For some applications (like bsdiff-like algorithms), this is an acceptable
tradeoff (the resulting patch will be slightly larger). For others, it isn't.

```rust
pub struct PartitionedSuffixArray<''a, Index> {
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
  pub fn new<F>(text: &''a [u8], num_partitions: usize, f: F) -> Self
where
    F: Fn(&''a [u8]) -> SuffixArray<''a, Index> + Sync { /* ... */ }
  ```

- ```rust
  pub fn num_partitions(self: &Self) -> usize { /* ... */ }
  ```

##### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **StringIndex**
  - ```rust
    fn longest_substring_match(self: &Self, needle: &[u8]) -> LongestCommonSubstring<''a> { /* ... */ }
    ```

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

