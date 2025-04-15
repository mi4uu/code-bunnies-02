# Crate Documentation

**Version:** 2.0.5

**Format Version:** 43

# Module `mime_guess`

Guessing of MIME types by file extension.

Uses a static list of file-extension : MIME type mappings.

```
# extern crate mime;
// the file doesn't have to exist, it just looks at the path
let guess = mime_guess::from_path("some_file.gif");
assert_eq!(guess.first(), Some(mime::IMAGE_GIF));

```

#### Note: MIME Types Returned Are Not Stable/Guaranteed
The media types returned for a given extension are not considered to be part of the crate's
stable API and are often updated in patch <br /> (`x.y.[z + 1]`) releases to be as correct as
possible.

Additionally, only the extensions of paths/filenames are inspected in order to guess the MIME
type. The file that may or may not reside at that path may or may not be a valid file of the
returned MIME type.  Be wary of unsafe or un-validated assumptions about file structure or
length.

## Types

### Struct `MimeGuess`

A "guess" of the MIME/Media Type(s) of an extension or path as one or more
[`Mime`](struct.Mime.html) instances.

### Note: Ordering
A given file format may have one or more applicable Media Types; in this case
the first Media Type returned is whatever is declared in the latest IETF RFC for the
presumed file format or the one that explicitly supercedes all others.
Ordering of additional Media Types is arbitrary.

### Note: Values Not Stable
The exact Media Types returned in any given guess are not considered to be stable and are often
updated in patch releases in order to reflect the most up-to-date information possible.

```rust
pub struct MimeGuess(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn from_ext(ext: &str) -> MimeGuess { /* ... */ }
  ```
  Guess the MIME type of a file (real or otherwise) with the given extension.

- ```rust
  pub fn from_path<P: AsRef<Path>>(path: P) -> MimeGuess { /* ... */ }
  ```
  Guess the MIME type of `path` by its extension (as defined by

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  `true` if the guess did not return any known mappings for the given path or extension.

- ```rust
  pub fn count(self: &Self) -> usize { /* ... */ }
  ```
  Get the number of MIME types in the current guess.

- ```rust
  pub fn first(self: &Self) -> Option<Mime> { /* ... */ }
  ```
  Get the first guessed `Mime`, if applicable.

- ```rust
  pub fn first_raw(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
  Get the first guessed Media Type as a string, if applicable.

- ```rust
  pub fn first_or_octet_stream(self: &Self) -> Mime { /* ... */ }
  ```
  Get the first guessed `Mime`, or if the guess is empty, return

- ```rust
  pub fn first_or_text_plain(self: &Self) -> Mime { /* ... */ }
  ```
  Get the first guessed `Mime`, or if the guess is empty, return

- ```rust
  pub fn first_or(self: &Self, default: Mime) -> Mime { /* ... */ }
  ```
  Get the first guessed `Mime`, or if the guess is empty, return the given `Mime` instead.

- ```rust
  pub fn first_or_else<F>(self: &Self, default_fn: F) -> Mime
where
    F: FnOnce() -> Mime { /* ... */ }
  ```
  Get the first guessed `Mime`, or if the guess is empty, execute the closure and return its

- ```rust
  pub fn iter(self: &Self) -> Iter { /* ... */ }
  ```
  Get an iterator over the `Mime` values contained in this guess.

- ```rust
  pub fn iter_raw(self: &Self) -> IterRaw { /* ... */ }
  ```
  Get an iterator over the raw media-type strings in this guess.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MimeGuess { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MimeGuess) -> bool { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Iter`

An iterator over the `Mime` types of a `MimeGuess`.

See [Note: Ordering on `MimeGuess`](struct.MimeGuess.html#note-ordering).

```rust
pub struct Iter(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
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

- **FusedIterator**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `IterRaw`

An iterator over the raw media type strings of a `MimeGuess`.

See [Note: Ordering on `MimeGuess`](struct.MimeGuess.html#note-ordering).

```rust
pub struct IterRaw(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IterRaw { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FusedIterator**
## Functions

### Function `from_ext`

Wrapper of [`MimeGuess::from_ext()`](struct.MimeGuess.html#method.from_ext).

```rust
pub fn from_ext(ext: &str) -> MimeGuess { /* ... */ }
```

### Function `from_path`

Wrapper of [`MimeGuess::from_path()`](struct.MimeGuess.html#method.from_path).

```rust
pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> MimeGuess { /* ... */ }
```

### Function `guess_mime_type`

**⚠️ Deprecated since 2.0.0**: Use `from_path(path).first_or_octet_stream()` instead

Guess the MIME type of `path` by its extension (as defined by `Path::extension()`).

If `path` has no extension, or its extension has no known MIME type mapping,
then the MIME type is assumed to be `application/octet-stream`.

## Note
**Guess** is the operative word here, as there are no guarantees that the contents of the file
that `path` points to match the MIME type associated with the path's extension.

Take care when processing files with assumptions based on the return value of this function.

In HTTP applications, it might be [preferable][rfc7231] to not send a `Content-Type`
header at all instead of defaulting to `application/octet-stream`.

[rfc7231]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5

```rust
pub fn guess_mime_type<P: AsRef<std::path::Path>>(path: P) -> Mime { /* ... */ }
```

### Function `guess_mime_type_opt`

**⚠️ Deprecated since 2.0.0**: Use `from_path(path).first()` instead

Guess the MIME type of `path` by its extension (as defined by `Path::extension()`).

If `path` has no extension, or its extension has no known MIME type mapping,
then `None` is returned.


```rust
pub fn guess_mime_type_opt<P: AsRef<std::path::Path>>(path: P) -> Option<Mime> { /* ... */ }
```

### Function `mime_str_for_path_ext`

**⚠️ Deprecated since 2.0.0**: Use `from_path(path).first_raw()` instead

Guess the MIME type string of `path` by its extension (as defined by `Path::extension()`).

If `path` has no extension, or its extension has no known MIME type mapping,
then `None` is returned.

## Note
**Guess** is the operative word here, as there are no guarantees that the contents of the file
that `path` points to match the MIME type associated with the path's extension.

Take care when processing files with assumptions based on the return value of this function.

```rust
pub fn mime_str_for_path_ext<P: AsRef<std::path::Path>>(path: P) -> Option<&''static str> { /* ... */ }
```

### Function `get_mime_type`

**⚠️ Deprecated since 2.0.0**: use `from_ext(search_ext).first_or_octet_stream()` instead

Get the MIME type associated with a file extension.

If there is no association for the extension, or `ext` is empty,
`application/octet-stream` is returned.

## Note
In HTTP applications, it might be [preferable][rfc7231] to not send a `Content-Type`
header at all instead of defaulting to `application/octet-stream`.

[rfc7231]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5

```rust
pub fn get_mime_type(search_ext: &str) -> Mime { /* ... */ }
```

### Function `get_mime_type_opt`

**⚠️ Deprecated since 2.0.0**: use `from_ext(search_ext).first()` instead

Get the MIME type associated with a file extension.

If there is no association for the extension, or `ext` is empty,
`None` is returned.

```rust
pub fn get_mime_type_opt(search_ext: &str) -> Option<Mime> { /* ... */ }
```

### Function `get_mime_type_str`

**⚠️ Deprecated since 2.0.0**: use `from_ext(search_ext).first_raw()` instead

Get the MIME type string associated with a file extension. Case-insensitive.

If `search_ext` is not already lowercase,
it will be converted to lowercase to facilitate the search.

Returns `None` if `search_ext` is empty or an associated extension was not found.

```rust
pub fn get_mime_type_str(search_ext: &str) -> Option<&''static str> { /* ... */ }
```

### Function `octet_stream`

**⚠️ Deprecated since 2.0.0**: use `mime::APPLICATION_OCTET_STREAM` instead

Get the MIME type for `application/octet-stream` (generic binary stream)

```rust
pub fn octet_stream() -> Mime { /* ... */ }
```

## Re-exports

### Re-export `Mime`

```rust
pub use mime::Mime;
```

## Other Items

### Extern Crate `mime`

```rust
pub extern crate mime;
```

