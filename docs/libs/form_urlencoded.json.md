# Crate Documentation

**Version:** 1.2.1

**Format Version:** 43

# Module `form_urlencoded`

Parser and serializer for the [`application/x-www-form-urlencoded` syntax](
http://url.spec.whatwg.org/#application/x-www-form-urlencoded),
as used by HTML forms.

Converts between a string (such as an URL’s query string)
and a sequence of (name, value) pairs.

## Types

### Struct `Parse`

The return type of `parse()`.

```rust
pub struct Parse<''a> {
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
  pub fn into_owned(self: Self) -> ParseIntoOwned<''a> { /* ... */ }
  ```
  Return a new iterator that yields pairs of `String` instead of pairs of `Cow<str>`.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parse<''a> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
### Struct `ParseIntoOwned`

Like `Parse`, but yields pairs of `String` instead of pairs of `Cow<str>`.

```rust
pub struct ParseIntoOwned<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

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

- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
### Struct `ByteSerialize`

Return value of `byte_serialize()`.

```rust
pub struct ByteSerialize<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Send**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a str> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Serializer`

The [`application/x-www-form-urlencoded` serializer](
https://url.spec.whatwg.org/#concept-urlencoded-serializer).

```rust
pub struct Serializer<''a, T: Target> {
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
  pub fn new(target: T) -> Self { /* ... */ }
  ```
  Create a new `application/x-www-form-urlencoded` serializer for the given target.

- ```rust
  pub fn for_suffix(target: T, start_position: usize) -> Self { /* ... */ }
  ```
  Create a new `application/x-www-form-urlencoded` serializer

- ```rust
  pub fn clear(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Remove any existing name/value pair.

- ```rust
  pub fn encoding_override(self: &mut Self, new: EncodingOverride<''a>) -> &mut Self { /* ... */ }
  ```
  Set the character encoding to be used for names and values before percent-encoding.

- ```rust
  pub fn append_pair(self: &mut Self, name: &str, value: &str) -> &mut Self { /* ... */ }
  ```
  Serialize and append a name/value pair.

- ```rust
  pub fn append_key_only(self: &mut Self, name: &str) -> &mut Self { /* ... */ }
  ```
  Serialize and append a name of parameter without any value.

- ```rust
  pub fn extend_pairs<I, K, V>(self: &mut Self, iter: I) -> &mut Self
where
    I: IntoIterator,
    <I as >::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str> { /* ... */ }
  ```
  Serialize and append a number of name/value pairs.

- ```rust
  pub fn extend_keys_only<I, K>(self: &mut Self, iter: I) -> &mut Self
where
    I: IntoIterator,
    <I as >::Item: Borrow<K>,
    K: AsRef<str> { /* ... */ }
  ```
  Serialize and append a number of names without values.

- ```rust
  pub fn finish(self: &mut Self) -> <T as >::Finished { /* ... */ }
  ```
  If this serializer was constructed with a string, take and return that string.

##### Trait Implementations

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Type Alias `EncodingOverride`

```rust
pub type EncodingOverride<''a> = Option<&''a dyn Fn(&str) -> alloc::borrow::Cow<''_, [u8]>>;
```

## Traits

### Trait `Target`

```rust
pub trait Target {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Finished`

##### Required Methods

- `as_mut_string`
- `finish`

#### Implementations

This trait is implemented for the following types:

- `alloc::string::String`
- `&''a mut alloc::string::String` with <''a>

## Functions

### Function `parse`

**Attributes:**

- `#[inline]`

Convert a byte string in the `application/x-www-form-urlencoded` syntax
into a iterator of (name, value) pairs.

Use `parse(input.as_bytes())` to parse a `&str` string.

The names and values are percent-decoded. For instance, `%23first=%25try%25` will be
converted to `[("#first", "%try%")]`.

```rust
pub fn parse(input: &[u8]) -> Parse<''_> { /* ... */ }
```

### Function `byte_serialize`

The [`application/x-www-form-urlencoded` byte serializer](
https://url.spec.whatwg.org/#concept-urlencoded-byte-serializer).

Return an iterator of `&str` slices.

```rust
pub fn byte_serialize(input: &[u8]) -> ByteSerialize<''_> { /* ... */ }
```

